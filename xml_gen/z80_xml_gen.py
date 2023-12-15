import pandas as pd
import numpy as np
import xml.etree.cElementTree as ET
from xml.dom import minidom

def splitargs(x):
    tokens = x.split(" ")
    if len(tokens) == 2:
        return tokens[1].split(",")
    if len(tokens) > 2:
        raise NotImplemented()

def clean_str(x):
    x = str(x)
    x = x.replace("²", "")
    x = x.replace("³", "")
    return x

def parse_csv(filename:str) -> pd.DataFrame:
    df = pd.read_csv(filename)
    df["instruction"] = df["instructionform"].apply(lambda x: x.split(" ")[0])
    df["arguments"] = df["instructionform"].apply(lambda x: splitargs(x))
    return df

def make_xml(df: pd.DataFrame, dst_filename: str):
    root = ET.Element("InstructionSet", name="z80")
    instruction_list = df["instruction"].unique().tolist()
    for instruction_name in instruction_list:
        instruction_elem = ET.Element("Instruction", name=instruction_name, summary="todo...")
        dff = df[df["instruction"] == instruction_name]
        for _, row in dff.iterrows():
            instructionform_elem = ET.Element("InstructionForm", upper=instruction_name.upper(), lower=instruction_name.lower())
            # opcodes
            opcodes = row["opcode"].split(" ")
            encoding_elem = ET.Element("Encoding")
            for opcode in opcodes:
                opcode_elem = ET.Element("Opcode", byte=opcode)
                encoding_elem.append(opcode_elem)
            instructionform_elem.append(encoding_elem) 
            # arguments and/or registers
            if row["arguments"] is not None and len(row["arguments"]) > 0:
                arg_elem = None
                args = row["arguments"][0].split(",")
                for arg in args:
                    if arg is None or arg is np.nan:
                        continue
                    arg_ = arg.replace("(", "")
                    arg_ = arg.replace(")", "")
                    if arg_ in "ABCDEHLIXIYSP":
                        arg_elem = ET.Element("Argument", reg=arg.lower())
                    else:
                        arg_elem = ET.Element("Argument", arg=arg.lower())
                if arg_elem is not None:
                    instructionform_elem.append(arg_elem)
            instruction_elem.append(instructionform_elem)
            instruction_elem.append(ET.Element("TimingZ80", value=clean_str(row["timing_z80"])))
            instruction_elem.append(ET.Element("TimingZ80M1", value=clean_str(row["timing_z80_m1"])))
            instruction_elem.append(ET.Element("TimingR800", value=clean_str(row["timing_r800"])))
            instruction_elem.append(ET.Element("TimingR800Wait", value=clean_str(row["timing_r800_wait"])))
        root.append(instruction_elem)
    tree = ET.ElementTree(root)
    tree.write(dst_filename)
    # Format the file to make it human readible.
    dom = minidom.parse(dst_filename)
    pretty_xml = dom.toprettyxml()
    with open(dst_filename, "w") as fp:
        fp.write(pretty_xml)
    print(f"Output written to {dst_filename}.")


if __name__ == "__main__":
    df = parse_csv("z80.csv")
    make_xml(df, "../opcodes/z80.xml")
