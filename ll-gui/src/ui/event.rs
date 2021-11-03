use gtk::MessageType;

#[derive(Debug)]
pub enum UiEvent {
    ShowInfoBar(String, MessageType),
    SwitchStack(&'static str),
    SetProfile(ll_core::Profile),
    UpdateAvailable,
    UpdateFormats,
}
