import { handleMIDIMessage, handleMIDIAccessGranted, handleMIDIAccessDenied} from "./midi.js"
import { updateSwitch, updateKnob, updateLeds, updateGenericKnob, updateScreen} from "./ui.js";

async function initialize() {
    let template = document.querySelector("#midi_device_item");
    let input_devices_element = document.querySelector("#midi_input_devices_container");
    let output_devices_element = document.querySelector("#midi_output_devices_container");
    
    try {
        const midi_access = await navigator.requestMIDIAccess({sysex: true, software: false});

        midi_access.onstatechange = (event) => {
            // Handle state change event
        }

        input_devices_element.innerHTML = ""; 

        for (const [id, device] of midi_access.inputs) {
            let midi_device_element = midi_item_element(template, device, (device) => {
                // When selecting a new input MIDI device
                // 0. We should remove any registered onmidimessage event handler
                for (const [id, device] of midi_access.inputs) {
                    delete device.onmidimessage;
                }
                // 1. Register an handler for onmidimessage event for the concerned midi device
                device.onmidimessage = (event) => {
                    handleMIDIMessage(event, (program) => {
                        // Render parts of the UI that depends on a onProgramEvent
                        updatePanel(program);
                    })
                } 
                // 1. MIDI input selected device should be updated
                render_selected_device_element("#midi_input_device_selected_item", device.name);

            });
            input_devices_element.appendChild(midi_device_element);
        }
        
        output_devices_element.innerHTML = "";
        for (const [id, device] of midi_access.outputs) {
            let midi_device_element = midi_item_element(template, device, (device) => {
                // When selecting a new output MIDI message
                // 0. The MIDI messages will be sent to the selected output MIDI device
                render_retrieve_patch("#menu-retrieve-patch", device, (device) => {
                    // Send the sysex message to the selected output
                    device.send([0xF0, 0x42, 0x3a, 0x00, 0x01, 0x4b, 0x10, 0xF7]);
                })
                // 1. MIDI output selected device should be updated
                render_selected_device_element("#midi_output_device_selected_item", device.name);

            });
            output_devices_element.appendChild(midi_device_element)
        }
    } catch (error) {
        console.log(error)
    }
}

function render_selected_device_element(querySelector, selected_device_name) {
    document.querySelector(querySelector).innerHTML = selected_device_name;
}

function render_save_patch_element(querySelector, program) {
    let program_name = program.program_name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
    let data = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(program, undefined, 2));
    let save_patch_el = document.querySelector(querySelector);
    save_patch_el.setAttribute("href", data);
    save_patch_el.setAttribute("download", program_name + ".json");
}

function render_retrieve_patch(querySelector, midiDevice, onClick) {
    let retrieve_patch_btn = document.querySelector(querySelector);
    
    let retrieve_patch_status = retrieve_patch_btn.querySelector(".connection_status").classList.replace("connection_status__notselected", "connection_status__disconnected");
    retrieve_patch_btn.classList.remove("menu_button_disabled");
    
    retrieve_patch_btn.addEventListener('click', event => {
        onClick(midiDevice)
    });
}

function midi_item_element(template, midiDevice, onClick) { 
    const component = template.content.cloneNode(true);
    component.querySelector("#midi_device_item__name").innerHTML = midiDevice.name;
    component.firstElementChild.addEventListener('click', event => {
        onClick(midiDevice);
        event.preventDefault(); 
    });
    return component;
}

function updatePanel(program) {
    // Remove panel opacity
    document.querySelector(".panel").classList.remove("panel-unavailable")

    // Update save patch 
    render_save_patch_element("#save_patch", program);

    // Update retrieve patch
    // document.querySelector("#menu-retrieve-patch-status").classList.replace("connection_status__disconnected", "connection_status__connected")
    //render_retrieve_patch()

    // Update Sections
    updateVoiceSection(program); 
    updateOscillatorsSection(program); // TODO 
    updateMixerSection(program); // TODO 
    updateFilterSection(program); // TODO 
    updateEnveloppeSection(program); // TODO 
    updateFXSection(program); // TODO 
}

function updateFXSection(program) {
    updateSwitch("#effect_mod .switch", program.mod_effect_enabled, w => false, w => w == false, w => w == true); 
    updateSwitch("#effect_reverbdelay .switch", program.delay_reverb_effect_enabled, w => false, w => w == false, w => w == true); 
    
    updateSwitch("#vcf-lowcut .switch", program.timbre1.low_cut_enabled, w => w, w => !w, w=>false);

    updateKnob("#effect_speed .knob", "#effect_speed .knob-value-label", program.mod_effect_speed);
    updateKnob("#effect_depth .knob", "#effect_depth .knob-value-label", program.mod_effect_depth);
    updateKnob("#effect_reverbdelay_time .knob", "#effect_reverbdelay_time .knob-value-label", program.delay_reverb_time);
    updateKnob("#effect_reverbdelay_depth .knob", "#effect_reverbdelay_depth .knob-value-label", program.delay_reverb_depth);

    // Update mod effects name
    if (program.mod_effect_enabled) {
        document.querySelector("#mod_effect_type").innerHTML = program.mod_effect_type;
    } else {
        document.querySelector("#mod_effect_type").innerHTML = "";
    }

    // Update time effects name
    if (program.delay_reverb_effect_enabled) {
        document.querySelector("#delayreverb_effect_type").innerHTML = program.delay_reverb_type;
    } else {
        document.querySelector("#delayreverb_effect_type").innerHTML = "";
    }

}

function updateFilterSection(program) {
    updateKnob("#vcf-cutoff .knob", "#vcf-cutoff .knob-value-label", program.timbre1.cutoff);
    updateKnob("#vcf-resonance .knob", "#vcf-resonance .knob-value-label", program.timbre1.resonance);
    updateGenericKnob("#vcf-egint .knob", "#vcf-egint .knob-value-label", program.timbre1.cutoff_eg_int, false, -4800, 4800);
    updateSwitch("#vcf-drive .switch", program.timbre1.cutoff_drive, w => w == "High", w => w == "Low", w => w == "Mid"); 
    updateSwitch("#vcf-lowcut .switch", program.timbre1.low_cut_enabled, w => w, w => !w, w=>false);
    updateSwitch("#vcf-keytrack .switch", program.timbre1.cutoff_keyboard_track, w => w == "High", w => w == "Low", w => w == "Mid");
}

function updateMixerSection(program) {
    updateKnob("#mixer-vco1-volume .knob", "#mixer-vco1-volume .knob-value-label", program.timbre1.vco1_level);
    updateKnob("#mixer-vco2-volume .knob", "#mixer-vco2-volume .knob-value-label", program.timbre1.vco2_level);
    updateKnob("#mixer-multi-volume .knob", "#mixer-multi-volume .knob-value-label", program.timbre1.multi_level);
}

function updateOscillatorsSection(program) {
    updateLeds("#vco1-octave .switch-labels", program.timbre1.vco_1_octave, d => d == 3, d => d == 2, d => d == 1, d => d == 0); // TODO
    updateLeds("#vco2-octave .switch-labels", program.timbre1.vco_2_octave, d => d == 3, d => d == 2, d => d == 1, d => d == 0); // TODO
    updateLeds("#multi-octave .switch-labels", program.timbre1.multi_octave, d => d == 3, d => d == 2, d => d == 1, d => d == 0); // TODO

    updateKnob("#vco1-shape .knob", "#vco1-shape .knob-value-label", program.timbre1.vco_1_shape);
    updateKnob("#vco2-shape .knob", "#vco2-shape .knob-value-label", program.timbre1.vco_2_shape);
    updateKnob("#multi-shape .knob", "#multi-shape .knob-value-label", program.timbre1.shape_noise);


    updateGenericKnob("#vco1-int .knob", "#vco1-int .knob-value-label", program.timbre1.pitch_eg_int, false, -4800, 4800);
    updateKnob("#cross-mod-depth .knob", "#cross-mod-depth .knob-value-label", program.timbre1.cross_mod_depth);

    updateKnob("#vco1-pitch .knob", "#vco1-pitch .knob-value-label", program.timbre1.vco_1_pitch);
    updateKnob("#vco2-pitch .knob", "#vco2-pitch .knob-value-label", program.timbre1.vco_2_pitch);

    updateSwitch("#vco1-wave .switch", program.timbre1.vco_1_wave, w => w=="Saw", w => w=="Square", w => w == "Triangle"); 
    updateSwitch("#vco2-wave .switch", program.timbre1.vco_2_wave, w => w=="Saw", w => w=="Square", w => w == "Triangle");

    updateSwitch("#vco1-pitch-eg .switch", program.timbre1.pitch_eg_target, w => w=="Plus", w => w=="Vco2", w => w == "Vco1");
    updateSwitch("#sync-ring .switch", program.timbre1.ring_sync, w => w=="Sync", w => w=="RingOn", w => w == "RingOff");
    updateSwitch("#multi-wave .switch", program.timbre1.multi_type, w => w=="User", w => w=="Noise", w => w == "Vpm");

    // Update screen 
    switch (program.timbre1.multi_type) {
        case "Vpm":
            updateScreen("#multi-screen > .screen", program.timbre1.select_vpm);
            break;
        case "Noise":
            updateScreen("#multi-screen > .screen", program.timbre1.select_noise);
            break;
        case "User":
            updateScreen("#multi-screen > .screen", program.timbre1.select_user);
            break;
    }

}

function updateEnveloppeSection(program) {    
    updateKnob("#lfo-rate .knob", "#lfo-rate .knob-value-label", program.timbre1.lfo_rate);
    updateKnob("#lfo-int .knob", "#lfo-int .knob-value-label", program.timbre1.lfo_int);

    updateKnob("#amp-attack .knob", "#amp-attack .knob-value-label", program.timbre1.amp_eg_attack);
    updateKnob("#amp-decay .knob", "#amp-decay .knob-value-label", program.timbre1.amp_eg_decay);
    updateKnob("#amp-sustain .knob", "#amp-sustain .knob-value-label", program.timbre1.amp_eg_sustain);
    updateKnob("#amp-release .knob", "#amp-release .knob-value-label", program.timbre1.amp_eg_release);

    updateKnob("#filter-eg-attack .knob", "#filter-eg-attack .knob-value-label", program.timbre1.eg_attack);
    updateKnob("#filter-eg-decay .knob", "#filter-eg-decay .knob-value-label", program.timbre1.eg_decay);
    updateKnob("#filter-eg-sustain .knob", "#filter-eg-sustain .knob-value-label", program.timbre1.eg_sustain);
    updateKnob("#filter-eg-release .knob", "#filter-eg-release .knob-value-label", program.timbre1.eg_release);

    updateSwitch("#lfo-wave .switch", program.timbre1.lfo_wave, w => w == "Saw", w => w == "Square", w => w == "Triangle");
    updateSwitch("#lfo-mode .switch", program.timbre1.lfo_mode, w => w=="Fast", w => w=="Bpm", w => w=="Slow"); // TODO 
    updateSwitch("#lfo-target .switch", program.timbre1.lfo_target, w => w=="Pitch", w => w=="Cutoff", w => w == "Shape"); // TODO 

}

function updateTimbreSection(program) {
    updateKnob("#timbre-mainsub-balance .knob", "#timbre-mainsub-balance .knob-value-label", program.main_sub_balance);
    updateSwitch("#timbretype .switch", program.timbre_type, w => w == "Split", w => w == "Layer", w => w == "Xfade"); // TODO 
}

function updateVoiceSection(program) {
    updateKnob("#portamento .knob", "#portamento .knob-value-label", program.timbre1.portamento_time);
    updateKnob("#voice-spread .knob", "#voice-spread .knob-value-label", program.timbre1.voice_spread);
    updateKnob("#voice-mode-depth .knob", "#voice-mode-depth .knob-value-label", program.timbre1.voice_mode_depth);
}

initialize();

document.querySelector(".full-screen-menu__close").addEventListener('click', (e) => {
    e.preventDefault();
    document.querySelector(".full-screen-menu").classList.toggle("full-screen-closed");
})

document.querySelector("#help").addEventListener('click', (e) => {
    e.preventDefault();
    document.querySelector(".full-screen-menu").classList.toggle("full-screen-closed");
})

let load_patch_link_el = document.querySelector("#load_patch");
let load_patch_input_el = document.querySelector("#load_patch_input");

load_patch_link_el.addEventListener("click", (event) => {
    if (load_patch_input_el) {
        load_patch_input_el.click();
    }
    event.preventDefault();
}, false);

load_patch_input_el.onchange = (event) => {
    let files = document.querySelector("#load_patch_input").files;
    files[0].text().then((t) => {
        let program = JSON.parse(t);
        updatePanel(program);
    });
};
