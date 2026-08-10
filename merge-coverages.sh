#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
output_path=${GIVN_COVERAGE_OUTPUT_PATH:-coverage/cobertura-coverage.xml}

python3 - "$non_e2e_path" "$e2e_path" "$output_path" <<'PY'
import copy
import pathlib
import sys
import xml.etree.ElementTree as ET

non_e2e_path, e2e_path, output_path = map(pathlib.Path, sys.argv[1:])
non_e2e = ET.parse(non_e2e_path).getroot()
e2e = ET.parse(e2e_path).getroot()

def class_map(root):
    result = {}
    for class_node in root.findall(".//class"):
        filename = class_node.attrib["filename"]
        lines = {
            line.attrib["number"]: int(line.attrib.get("hits", 0))
            for line in class_node.findall("./lines/line")
        }
        result[filename] = (class_node, lines)
    return result

non_e2e_classes = class_map(non_e2e)
e2e_classes = class_map(e2e)
all_filenames = set(non_e2e_classes) | set(e2e_classes)

def merged_lines(filename):
    lines = {}
    for classes in (non_e2e_classes, e2e_classes):
        if filename in classes:
            for number, hits in classes[filename][1].items():
                lines[number] = max(lines.get(number, 0), hits)
    return lines

def update_class(class_node, lines):
    line_nodes = {line.attrib["number"]: line for line in class_node.findall("./lines/line")}
    for number, hits in lines.items():
        line_node = line_nodes.get(number)
        if line_node is None:
            lines_node = class_node.find("./lines")
            if lines_node is None:
                lines_node = ET.SubElement(class_node, "lines")
            line_node = ET.SubElement(lines_node, "line", number=number)
        line_node.set("hits", str(hits))
    covered = sum(hits > 0 for hits in lines.values())
    valid = len(lines)
    class_node.set("lines-covered", str(covered))
    class_node.set("lines-valid", str(valid))
    class_node.set("line-rate", str(covered / valid if valid else 0.0))

for filename in all_filenames:
    lines = merged_lines(filename)
    if filename in non_e2e_classes:
        update_class(non_e2e_classes[filename][0], lines)
    else:
        source_class = copy.deepcopy(e2e_classes[filename][0])
        package_name = source_class.attrib["name"].rsplit(".", 1)[0]
        package = next(
            (node for node in non_e2e.findall("./packages/package") if node.attrib["name"] == package_name),
            None,
        )
        if package is None:
            packages = non_e2e.find("./packages")
            package = ET.SubElement(packages, "package", name=package_name, complexity="0")
        package.find("./classes").append(source_class)
        update_class(source_class, lines)

for package in non_e2e.findall("./packages/package"):
    package_lines = package.findall("./classes/class")
    covered = sum(int(node.attrib.get("lines-covered", 0)) for node in package_lines)
    valid = sum(int(node.attrib.get("lines-valid", 0)) for node in package_lines)
    package.set("lines-covered", str(covered))
    package.set("lines-valid", str(valid))
    package.set("line-rate", str(covered / valid if valid else 0.0))

lines_covered = sum(int(node.attrib.get("lines-covered", 0)) for node in non_e2e.findall(".//class"))
lines_valid = sum(int(node.attrib.get("lines-valid", 0)) for node in non_e2e.findall(".//class"))
branches_covered = sum(int(node.attrib.get("branches-covered", 0)) for node in non_e2e.findall(".//class"))
branches_valid = sum(int(node.attrib.get("branches-valid", 0)) for node in non_e2e.findall(".//class"))
non_e2e.attrib.update({
    "lines-covered": str(lines_covered),
    "lines-valid": str(lines_valid),
    "branches-covered": str(branches_covered),
    "branches-valid": str(branches_valid),
    "line-rate": str(lines_covered / lines_valid if lines_valid else 0.0),
    "branch-rate": str(branches_covered / branches_valid if branches_valid else 0.0),
})

output_path.parent.mkdir(parents=True, exist_ok=True)
ET.ElementTree(non_e2e).write(output_path, encoding="utf-8", xml_declaration=True)
PY
