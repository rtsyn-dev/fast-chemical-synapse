use rtsyn_plugin::prelude::*;
use serde_json::Value;

#[derive(Debug)]
struct FastChemicalSynapse {
    in_0: f64,
    in_1: f64,
    out_0: f64,
    int_0: f64,
    int_1: f64,
    int_2: f64,
    int_3: f64,
}

impl Default for FastChemicalSynapse {
    fn default() -> Self {
        Self {
            in_0: 0.0,
            in_1: 0.0,
            out_0: 0.0,
            int_0: 0.0,
            int_1: 0.0,
            int_2: 0.0,
            int_3: 0.0,
        }
    }
}

impl PluginDescriptor for FastChemicalSynapse {
    fn name() -> &'static str {
        "Fast Chemical Synapse"
    }

    fn kind() -> &'static str {
        "fast_chemical_synapse"
    }

    fn plugin_type() -> PluginType {
        PluginType::Standard
    }

    fn inputs() -> &'static [&'static str] {
        &["pre", "post"]
    }

    fn outputs() -> &'static [&'static str] {
        &["i_syn"]
    }

    fn internal_variables() -> &'static [&'static str] {
        &["g_fast", "e_syn", "s_fast", "v_fast"]
    }

    fn default_vars() -> Vec<(&'static str, Value)> {
        Vec::new()
    }

    fn behavior() -> PluginBehavior {
        PluginBehavior {
            supports_start_stop: true,
            supports_restart: true,
            supports_apply: false,
            extendable_inputs: ExtendableInputs::None,
            loads_started: false,
            external_window: false,
            starts_expanded: true,
            start_requires_connected_inputs: Vec::new(),
            start_requires_connected_outputs: Vec::new(),
        }
    }
}

impl PluginRuntime for FastChemicalSynapse {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        match key {
            "g_fast" => self.int_0 = value.as_f64().unwrap_or(self.int_0),
            "e_syn" => self.int_1 = value.as_f64().unwrap_or(self.int_1),
            "s_fast" => self.int_2 = value.as_f64().unwrap_or(self.int_2),
            "v_fast" => self.int_3 = value.as_f64().unwrap_or(self.int_3),
            _ => {}
        }
    }

    fn set_input_value(&mut self, key: &str, v: f64) {
        match key {
            "pre" => self.in_0 = if v.is_finite() { v } else { 0.0 },
            "post" => self.in_1 = if v.is_finite() { v } else { 0.0 },
            _ => {}
        }
    }

    fn process_tick(&mut self, _tick: u64, period_seconds: f64) {
        let _ = period_seconds;
        let first_input = self.in_0;
        self.out_0 = first_input;
    }

    fn get_output_value(&self, key: &str) -> f64 {
        match key {
            "i_syn" => self.out_0,
            _ => 0.0,
        }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        match key {
            "g_fast" => Some(self.int_0),
            "e_syn" => Some(self.int_1),
            "s_fast" => Some(self.int_2),
            "v_fast" => Some(self.int_3),
            _ => None,
        }
    }
}

rtsyn_plugin::export_plugin!(FastChemicalSynapse);
