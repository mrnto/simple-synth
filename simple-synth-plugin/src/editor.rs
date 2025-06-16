use std::sync::Arc;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use crate::SimpleSynthParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(800, 640)
}

pub(crate) fn create(
    params: Arc<SimpleSynthParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<SimpleSynthEditor>(editor_state, params)
}

struct SimpleSynthEditor {
    params: Arc<SimpleSynthParams>,
    context: Arc<dyn GuiContext>,
    gain_slider_state: nih_widgets::param_slider::State,
    attack_slider_state: nih_widgets::param_slider::State,
    decay_slider_state: nih_widgets::param_slider::State,
    sustain_slider_state: nih_widgets::param_slider::State,
    release_slider_state: nih_widgets::param_slider::State,
    cutoff_slider_state: nih_widgets::param_slider::State,
    resonance_slider_state: nih_widgets::param_slider::State,
    filter_env_amount_slider_state: nih_widgets::param_slider::State,
    filter_attack_slider_state: nih_widgets::param_slider::State,
    filter_decay_slider_state: nih_widgets::param_slider::State,
    filter_sustain_slider_state: nih_widgets::param_slider::State,
    filter_release_slider_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for SimpleSynthEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<SimpleSynthParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = SimpleSynthEditor {
            params,
            context,
            gain_slider_state: Default::default(),
            attack_slider_state: Default::default(),
            decay_slider_state: Default::default(),
            sustain_slider_state: Default::default(),
            release_slider_state: Default::default(),
            cutoff_slider_state: Default::default(),
            resonance_slider_state: Default::default(),
            filter_env_amount_slider_state: Default::default(),
            filter_attack_slider_state: Default::default(),
            filter_decay_slider_state: Default::default(),
            filter_sustain_slider_state: Default::default(),
            filter_release_slider_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let attack_slider = nih_widgets::ParamSlider::new(&mut self.attack_slider_state, &self.params.attack)
            .map(Message::ParamUpdate);
        let decay_slider = nih_widgets::ParamSlider::new(&mut self.decay_slider_state, &self.params.decay)
            .map(Message::ParamUpdate);
        let sustain_slider = nih_widgets::ParamSlider::new(&mut self.sustain_slider_state, &self.params.sustain)
            .map(Message::ParamUpdate);
        let release_slider = nih_widgets::ParamSlider::new(&mut self.release_slider_state, &self.params.release)
            .map(Message::ParamUpdate);

        let cutoff_slider = nih_widgets::ParamSlider::new(&mut self.cutoff_slider_state, &self.params.cutoff)
            .map(Message::ParamUpdate);
        let resonance_slider = nih_widgets::ParamSlider::new(&mut self.resonance_slider_state, &self.params.resonance)
            .map(Message::ParamUpdate);
        let filter_env_amount_slider = nih_widgets::ParamSlider::new(&mut self.filter_env_amount_slider_state, &self.params.filter_env_amount)
            .map(Message::ParamUpdate);
        let filter_attack_slider = nih_widgets::ParamSlider::new(&mut self.filter_attack_slider_state, &self.params.filter_attack)
            .map(Message::ParamUpdate);
        let filter_decay_slider = nih_widgets::ParamSlider::new(&mut self.filter_decay_slider_state, &self.params.filter_decay)
            .map(Message::ParamUpdate);
        let filter_sustain_slider = nih_widgets::ParamSlider::new(&mut self.filter_sustain_slider_state, &self.params.filter_sustain)
            .map(Message::ParamUpdate);
        let filter_release_slider = nih_widgets::ParamSlider::new(&mut self.filter_release_slider_state, &self.params.filter_release)
            .map(Message::ParamUpdate);

        let env_row = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Attack"))
                .push(attack_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Decay"))
                .push(decay_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Sustain"))
                .push(sustain_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Release"))
                .push(release_slider)
            );

        let filter_row = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            // .push(Column::new()
            //     .push(Text::new("Filter Mode"))
            //     .push(filter_mode_dropdown)
            // )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Cutoff"))
                .push(cutoff_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Resonance"))
                .push(resonance_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Filter Env Amount"))
                .push(filter_env_amount_slider)
            );

        let filter_env_row = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Attack"))
                .push(filter_attack_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Decay"))
                .push(filter_decay_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Sustain"))
                .push(filter_sustain_slider)
            )
            .push(Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("Release"))
                .push(filter_release_slider)
            );

        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("SimpleSynth GUI")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new("Gain")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            // .push(Space::with_height(10.into()))
            // .push(Column::new()
            //     .push(Text::new("Waveform"))
            //     .push(waveform_dropdown)
            // )
            .push(Space::with_height(10.into()))
            .push(env_row)
            .push(Space::with_height(10.into()))
            .push(filter_row)
            .push(Space::with_height(10.into()))
            .push(filter_env_row)
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color { r: 0.98, g: 0.98, b: 0.98, a: 1.0 }
    }
}
