import json
import re

with open("json/aarch32.json") as f:
    db = json.load(f)

out = {
    "arm": {},
    "thumb": {},
}


def split_operands(tail: str):
    tokens = []
    curr = []
    depth = 0
    for ch in tail:
        if ch == " " and depth == 0:
            if curr:
                tokens.append("".join(curr))
                curr = []
        else:
            curr.append(ch)
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth = max(depth - 1, 0)
    if curr:
        tokens.append("".join(curr))
    return tokens


def parse_operands(syntax_text: str):
    parts = syntax_text.strip().split(None, 1)
    if len(parts) < 2:
        return []
    tail = parts[1]

    raw_tokens = split_operands(tail)
    operands = []
    for tok in raw_tokens:
        is_hex = tok.startswith("0x") and re.fullmatch(r"0x[0-9A-Fa-f]+", tok)
        is_dec = tok.isdigit()
        is_hash = tok.startswith("#")
        is_immnm = re.match(r"^imm(\d+)$", tok)

        immediate = bool(is_hex or is_dec or is_hash or is_immnm)

        entry = {"name": tok, "immediate": immediate}

        if immediate:
            if is_hex:
                hex_digits = tok[2:]
                entry["bytes"] = (len(hex_digits) + 1) // 2
            elif is_dec or (is_hash and tok[1:].is_digit()):
                entry["bytes"] = 1
            elif is_immnm:
                bits = int(is_immnm.group(1))
                entry["bytes"] = (bits + 7) // 8

        operands.append(entry)
    return operands


for rec in db["records"]:
    if rec.get("rectype") != "ENCODING":
        continue

    isa = rec.get("metadata", {}).get("isaform", "")
    section = "arm" if isa.startswith("A") else "thumb"
    table = out[section]

    opcode = rec["diagram"]["opcode"]
    size = rec["diagram"]["size"]
    hexkey = f"0x{opcode:0{size//4}X}"

    tmpl = rec.get("templates", [])
    if not tmpl:
        continue
    syn = tmpl[0]["syntax"]
    text = syn.get("text", "").strip()
    mnem = syn.get("mnem", "").strip()

    operands = parse_operands(text)

    table[hexkey] = {
        "mnemonic": mnem,
        "bytes": size // 8,
        "cycles": [],
        "operands": operands,
        "immediate": any(o["immediate"] for o in operands),
        "flags": {
            "Z": "-",
            "N": "-",
            "H": "-",
            "C": "-"
        }
    }

with open("data/arm_thumb_opcodes.json", "w") as f:
    json.dump(out, f, indent=2)

print("-> data/arm_thumb_opcodes.json generated")
