const ops = (cb) => {
  let table = document.querySelector(`body > table:nth-child(${cb ? 10 : 4})`);
  return [...table.querySelectorAll("td")]
    .map((x) => {
      let kind = "";
      let bgcolor = document.defaultView
        .getComputedStyle(x, null)
        .getPropertyValue("background-color");
      switch (bgcolor) {
        case "rgb(255, 153, 204)":
          kind = "InstructionKind::Control";
          break;
        case "rgb(255, 204, 153)":
          kind = "InstructionKind::Jump";
          break;
        case "rgb(204, 204, 255)":
          kind = "InstructionKind::LSM8Bit";
          break;
        case "rgb(204, 255, 204)":
          kind = "InstructionKind::LSM16Bit";
          break;
        case "rgb(255, 255, 153)":
          kind = "InstructionKind::AL8Bit";
          break;
        case "rgb(255, 204, 204)":
          kind = "InstructionKind::AL16Bit";
          break;
        case "rgb(128, 255, 255)":
          kind = "InstructionKind::RSB8Bit";
          break;
      }
      if (x.innerHTML === "&nbsp;") {
        return ["NI", 0, 0, 0, cb, "InstructionKind::NI"];
      }
      x = x.innerText.split("\n");
      if (/[A-Z]/.test(x[0][0])) {
        let a = x[1].split("\u00A0");
        let size = a[0];
        let cycles = a[2];
        let cycles_not_taken = 0;
        if (a[2].includes("/")) {
          [cycles, cycles_not_taken] = a[2].split("/");
        }
        if (cb) {
          // table includes cycles and size of the `PREFIX CB` instruction.
          // internally we also count `PREFIX CB` individually, so subtract it.
          cycles -= 4;
          size -= 1;
        }
        return [x[0], size, cycles, cycles_not_taken, cb, kind];
      }
      return undefined;
    })
    .filter((x) => x !== undefined)
    .slice(1);
};

const instruction = (x, i) => {
  console.log(
    `Instruction { opcode: 0x${x[4] ? "1" : ""}${i.toString(16).padStart(2, "0").toUpperCase()}, mnemonic: "${x[0]}", size: ${x[1]}, cycles: ${x[2]}, cycles_not_taken: ${x[3]}, kind: ${x[5]} },`,
  );
};

ops(false).forEach((x, i) => instruction(x, i));
ops(true).forEach((x, i) => instruction(x, i));

//const match = (x, i) => {
//  console.log(`0x${x[4]?'1':''}${i.toString(16).padStart(2,'0').toUpperCase()} => {\n// ${x[0]}\nunimplemented!()}`);
//}
//
//ops(false).forEach((x, i) => match(x, i));
//ops(true).forEach((x, i) => match(x, i));
//
