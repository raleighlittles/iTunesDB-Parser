import argparse
import os
import lxml.etree
import pdb


if __name__ == "__main__":

    argparse_parser = argparse.ArgumentParser()
    argparse_parser.add_argument("-i", "--input-itunes-xml-file", help="Path to the XML file exported by iTunes", type=str)
    argparse_args = argparse_parser.parse_args()

    if not os.path.exists(argparse_args.input_itunes_xml_file):
        raise FileNotFoundError(f"ERROR! iTunes XML Library file not found '{argparse_args.input_itunes_xml_file}'")
    
    itunes_xml_obj = lxml.etree.parse(argparse_args.input_itunes_xml_file)
    itunes_xml_root = itunes_xml_obj.getroot()

    pdb.set_trace()