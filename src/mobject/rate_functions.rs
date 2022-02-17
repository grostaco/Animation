use super::sigmoid;

pub fn smooth(t: f32, inflection: f32) -> f32 {
    let error = sigmoid(-inflection / 2.);
    ((sigmoid(inflection * (t - 0.5)) - error) / (1. - 2. * error)).clamp(0., 1.)
}
