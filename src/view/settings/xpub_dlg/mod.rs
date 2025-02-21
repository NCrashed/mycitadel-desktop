// MyCitadel desktop wallet: bitcoin & RGB wallet based on GTK framework.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoraprime.ch>
//
// Copyright (C) 2022 by Pandora Prime Sarl, Switzerland.
//
// This software is distributed without any warranty. You should have received
// a copy of the AGPL-3.0 License along with this software. If not, see
// <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

mod component;
mod view_model;
mod widget;

pub use component::Component;
use gtk::ResponseType;
pub(self) use view_model::ViewModel;
use wallet::hd::Bip43;
pub(self) use widget::Widgets;

#[derive(Clone, Msg)]
pub enum Msg {
    Open(bool, Bip43),
    Edit,
    Error(String),
    Warning(String),
    Info(String),
    Response(ResponseType),
}
