use chrono::{DateTime, Local};
use egui::{Color32, Label, Stroke, TextStyle};

use crate::TemplateApp;

pub struct Table {
    striped: bool,
   
 
   
}
impl Default for Table {
    fn default() -> Self {
        Self {
           
            striped: true,
          
        
            
        }
    }
}

impl Table {
    
   pub fn table_ui(&mut self, ui: &mut egui::Ui,table_data:&mut TemplateApp) {
        use egui_extras::{Column, TableBuilder};

   
    let s = Stroke {
            width: 0.0,
            color: Color32::TRANSPARENT,
        };
    let available_height = ui.available_height();
    let mut table = TableBuilder::new(ui).cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
        .column(Column::auto())
        .column(Column::exact(80.00))
        .column(Column::exact(80.00))
        .column(Column::exact(80.00))
        .resizable(false)
        .striped(self.striped)
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);
        
        table = table.sense(egui::Sense::click());
        if let Some(row_index) = table_data.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }
    
                
        
    table.header(20.0, |mut header| {
        header.col(|ui| {
         
        let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Order#").fill(egui::Color32::TRANSPARENT));
        if sort_click.clicked(){
            let data_temp= table_data.total_order.clone();
            table_data.total_order.sort();
            if data_temp== table_data.total_order{
                table_data.total_order.reverse();
            }
          
        }

        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Check In").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.clone();
                table_data.total_order.sort_by_key(|k| k.1);
                if data_temp== table_data.total_order{
                    table_data.total_order.reverse();
                }
              
            }
         

        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Wait Time").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.clone();
                table_data.total_order.sort_by_key(|k| k.1);
                if data_temp== table_data.total_order{
                    table_data.total_order.reverse();
                }
              
            }
        });
        header.col(|ui| {
          ui.add_sized(ui.available_size(),egui::Button::new("Payment").fill(egui::Color32::TRANSPARENT));
     
        })
        ;
        })
        
    .body(|mut body| {
    let order_size=table_data.total_order.len();
    for row_index in 0..order_size {    
        if row_index>=table_data.total_order.len(){
            continue;
         }
        body.row(20.0, |mut row| {
            let rowindex=row.index();
            if table_data.selection==row_index{
                row.set_selected(true);
            }else {
                row.set_selected(false);
            }
            
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.total_order[rowindex].0.clone()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.total_order[rowindex].1.clone().format("%H:%M").to_string()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let time_now: DateTime<Local> = Local::now();
                let time_wait = time_now-table_data.total_order[rowindex].1.clone();
                let minutes = (time_wait.num_minutes()).to_string();
               
               
               let time= minutes+" min";
                ui.add_sized( ui.available_size(),Label::new(egui::RichText::new(time).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let response = ui
                .add_sized(
                    ui.available_size(),
                    egui::Button::new(if table_data.total_order[rowindex].2 {"Paid"}else{""}),
                );
                if response.clicked(){
                 
                    table_data.total_order[rowindex].2 =!table_data.total_order[rowindex].2;
                  
                }
            });
            
            toggle_row_selection(table_data,row_index, &row.response());
         
        });
       
        };
        body.row(20.0, |mut row| {
            if table_data.selection==table_data.total_order.len(){
                row.set_selected(true);
            }else {
                row.set_selected(false);
            }
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.order_number.concat()).size(20.0)).selectable(false),);
            });
           

        });
        
   
       
    });
    }
  
}  
fn toggle_row_selection(select:&mut TemplateApp, row_index: usize, row_response: &egui::Response) {
    if row_response.clicked() {
        select.selection=row_index;
    }
    if row_response.double_clicked() {
     
        select.total_order.remove(row_index);
        select.payment.remove(row_index);
        select.payment.push(false);
        select.selection=999
    }
}
