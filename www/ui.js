function updateSwitch(dom_selector, data, map_to_high, map_to_low, map_to_mid) {
  let dom_elem = document.querySelector(dom_selector);
  dom_elem.classList.remove("switch-off");
  dom_elem.classList.remove("switch-on");
  dom_elem.classList.remove("switch-mid");

  if (map_to_high(data)) {
    dom_elem.classList.add("switch-on");
    return;
  }

  if (map_to_low(data)) {
    dom_elem.classList.add("switch-off");
    return;
  }

  if (map_to_mid(data)) {
    dom_elem.classList.add("switch-mid");
    return;
  }
}

function updateGenericKnob(
  dom_selector,
  dom_selector_label,
  data,
  is_unipolar,
  output_start,
  output_end
) {
  let MIN_ANGLE = 120;
  let MAX_ANGLE = 120;
  let a = MAX_ANGLE + MIN_ANGLE;
  let b = -MIN_ANGLE;

  // Update label
  document.querySelector(dom_selector_label).innerHTML = data;

  // Update knob rotation
  let angle;
  if (is_unipolar) {
    angle = a * (data / (output_end - output_start)) + b;
  } else {
    angle = MAX_ANGLE * (data / (output_end - output_start));
  }

  document.querySelector(dom_selector).style.transform =
    "rotate(" + angle + "deg)";
}

function updateKnob(dom_selector, dom_selector_label, data) {
  updateGenericKnob(dom_selector, dom_selector_label, data, true, 0, 1023);
}

function updateLeds(dom_selector, data, ...led_activation_functions) {
  let dom_leds = document.querySelector(dom_selector).children;

  // Reset led states
  for (const dom_led of dom_leds) {
    dom_led.classList.remove("switch-state-selected");
  }

  // Go through activation functions to turn on leds.
  for (const [
    i,
    led_activation_function,
  ] of led_activation_functions.entries()) {
    if (led_activation_function(data)) {
      dom_leds[i].classList.add("switch-state-selected");
      break;
    }
  }
}

function updateScreen(dom_selector, text) {
  document.querySelector(dom_selector).innerText = text;
}

export {
  updateSwitch,
  updateKnob,
  updateLeds,
  updateGenericKnob,
  updateScreen,
};
