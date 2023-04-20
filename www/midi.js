import * as wasm from "patchlogue";
import { isPrologueCurrentProgramDumpSysexMessage } from "./utils.js";

function handleMIDIMessage(event, onPatchLoadedCallback) {
  if (isPrologueCurrentProgramDumpSysexMessage(event.data)) {
    handleCurrentProgramDumpSysexMessage(event.data, onPatchLoadedCallback);
  }
}

function handleCurrentProgramDumpSysexMessage(data, onPatchLoadedCallback) {
  const json_result_from_wasm = wasm.read_program(data);
  let program = JSON.parse(json_result_from_wasm);
  onPatchLoadedCallback(program);
}

export { handleMIDIMessage };
