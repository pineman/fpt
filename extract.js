const ops = (cb) => {
	let table = document.querySelector(`body > table:nth-child(${cb?10:4})`)
  return [...table.querySelectorAll('td')].map((x) => {
    if (x.innerHTML === '&nbsp;') {
      return ['NOTHING', 0, 0, 0];
    }
    x = x.innerText.split('\n');
    if (/[A-Z]/.test(x[0][0])) {
    	let a = x[1].split('\u00A0')
    	let size = a[0]
    	let cycles = a[2];
    	let cycles_not_taken = 0;
    	if (a[2].includes('/')) {
				[cycles, cycles_not_taken] = a[2].split('/')
			}
			if (cb) {
				// table includes cycles and size of the `PREFIX CB` instruction.
				// internally we also count `PREFIX CB` individually, so subtract it.
				cycles -= 4
				size -= 1
			}
      return [x[0], size, cycles, cycles_not_taken]
    }
    return undefined;
  }).filter((x) => (x !== undefined)).slice(1)
}

const instruction = (x, i) => {
  console.log(`Instruction::new(0x${i.toString(16).toUpperCase()}, "${x[0]}", ${x[1]}, ${x[2]}, ${x[3]}),`);
}

ops(false).forEach((x, i) => instruction(x, i));
ops(true).forEach((x, i) => instruction(x, i));

const match = (x, i) => {
  console.log(`0x${i.toString(16).toUpperCase()} => unimplemented!(),`);
}

ops(false).forEach((x, i) => match(x, i));
ops(true).forEach((x, i) => match(x, i));

