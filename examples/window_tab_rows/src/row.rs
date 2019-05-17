use sauron::{html::{attributes::*,
                    events::*,
                    *},
             Component,
             Node};

use crate::field::{self,
                   Field};

use sauron::Cmd;

#[derive(Debug, Clone)]
pub enum Msg {
    RowClick,
    FieldMsg(usize, field::Msg),
}

pub struct Row {
    row_clicks: u32,
    fields: Vec<Field>,
    row_name: String,
}

impl Row {
    pub fn new(row_name: String) -> Self {
        Row { row_clicks: 0,
              fields:
                  (0..10).into_iter()
                         .map(|index| Field::new(format!("Field {}", index)))
                         .collect(),
              row_name }
    }
}

impl Component<Msg> for Row {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::RowClick => {
                self.row_clicks += 1;
            }
            Msg::FieldMsg(index, field_msg) => {
                self.fields[index].update(field_msg);
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div([class("row"), onclick(|_| Msg::RowClick)],
            [text(&self.row_name),
             input([class("row-selector"), r#type("checkbox")], []),
             div([],
                 self.fields
                     .iter()
                     .enumerate()
                     .map(|(index, field)| {
                         field.view().map(move |field_msg| {
                                         Msg::FieldMsg(index, field_msg)
                                     })
                     })
                     .collect::<Vec<Node<Msg>>>()),
             span([],
                  [text(format!("total activities: {}", self.row_clicks))])])
    }
}
