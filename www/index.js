import {handleMIDIMessage, handleMIDIAccessGranted, handleMIDIAccessDenied} from "./midi.js"
import { updateSwitch, updateKnob, updateLeds, updateGenericKnob, updateScreen} from "./ui.js";

navigator.requestMIDIAccess({sysex: true, software: true})
    .then((midiAccess) => {
        // Populate the list of MIDI inputs in the UI.
        let input_devices_element = document.querySelector("#midi_input_devices_container");
        input_devices_element.innerHTML = ""; // Resets the HTML
        var template = document.querySelector("#midi_device_item");
        for (const [id, device] of midiAccess.inputs) {
            const clone = template.content.cloneNode(true);
            clone.querySelector("#midi_device_item__name").innerHTML = device.name;
            clone.firstElementChild.addEventListener('click', event => {
                document.querySelector("#midi_input_device_selected_item").innerHTML = device.name;
                device.onmidimessage = (event) => {
                    handleMIDIMessage(event, (program) => { 
                        let program_name = program.program_name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
                        let data = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(program, undefined, 2));
                        let save_patch_el = document.querySelector("#save_patch");
                        save_patch_el.setAttribute("href", data);
                        save_patch_el.setAttribute("download", program_name + ".json");                    
                        updateUI(program);
                    })
                };
            })
            input_devices_element.appendChild(clone);
        }

        // Populate the list of MIDI outputs in the UI. 
        let output_devices_element = document.querySelector("#midi_output_devices_container");
        output_devices_element.innerHTML = ""; // Resets the HTML
        var template = document.querySelector("#midi_device_item");
        for (const [id, device] of midiAccess.outputs) {
            const clone = template.content.cloneNode(true);
            clone.querySelector("#midi_device_item__name").innerHTML = device.name;
            clone.firstElementChild.addEventListener('click', event => {
                // Mandatory : Display the name of the selected device in the UI. 
                document.querySelector("#midi_output_device_selected_item").innerHTML = device.name;
                // Bind actions to current midi device. 
                let retrieve_patch_btn = document.querySelector("#menu-retrieve-patch");
                let retrieve_patch_status = document.querySelector("#menu-retrieve-patch-status").classList.replace("connection_status__notselected", "connection_status__disconnected");

                retrieve_patch_btn.parentNode.classList.remove("menu_button_disabled");
                retrieve_patch_btn.addEventListener('click', event => {
                    console.log("sending prog");
                    device.send([0xF0, 0x42, 0x3a, 0x00, 0x01, 0x4b, 0x10, 0xF7]); // Send current program data dump req

                })
            });
            output_devices_element.appendChild(clone);
        }

    }, 
    handleMIDIAccessDenied);


/*navigator.requestMIDIAccess({sysex: true, software: true})
    .then(
        (midiAccess) => { 
            handleMIDIAccessGranted(midiAccess, (program) => {
                updateUI(program)
            }, 
            (midi_devices) => {
                let devices_element = document.querySelector("#midi_devices_container");
                devices_element.innerHTML = ""; 
                let template = document.querySelector("#midi_device_item");
                for (const [id, device] of midi_devices) {
                    const clone = template.content.cloneNode(true);
                    clone.querySelector("#midi_device_item__name").innerHTML = device.name;
                    clone.firstElementChild.addEventListener('click', event => {
                        // Make this device the preferred device to send MIDI data to. 
                        
                        // Mandatory : Display the name of the selected device in the UI. 
                        document.querySelector("#midi_device_selected_item").innerHTML = device.name;
                        
                        // Bind actions to current midi device. 
                        let connection_status_el = document.querySelector("#connection_status");
                        connection_status_el.classList.replace("connection_status__notselected", "connection_status__disconnected");
                        connection_status_el.classList.add("connection_status__disconnected");

                        console.log(device);
                        // Let's send a Search Device Reply Message to all outputs







                    });
                    devices_element.appendChild(clone);
                }
            }
    )}, 
    handleMIDIAccessDenied);*/

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
        updateUI(program);
    });
};

function updateUI(program){
    document.querySelector("#menu-retrieve-patch-status").classList.replace("connection_status__disconnected", "connection_status__connected")
    document.querySelector(".panel").classList.remove("panel-unavailable")
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


