export function isPrologueCurrentProgramDumpSysexMessage(data) {
  return (
    data[0] === 0xf0 &&
    data[1] === 0x42 &&
    data[2] === 0x3a &&
    data[3] === 0x00 &&
    data[4] === 0x01 &&
    data[5] === 0x4b &&
    data[6] === 0x40
  );
}
