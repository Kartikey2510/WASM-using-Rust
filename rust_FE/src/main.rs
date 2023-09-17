use yew::*; 
use web_sys::{HtmlInputElement};
use wasm_bindgen::*;

enum Msg{
    Add,
    Subtract,
    Multiply,
    Divide,
}
struct Result{
    result : i64,
}

impl Component for Result{
    type Message = Msg;
    type Properties = ();

     fn create(ctx: &Context<Self>) -> Self {
        Self { 
        result: 0 
    }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        let mut targeta = "10".to_string();              //TODO read this from user
        let mut targetb = "2".to_string();               //TODO read this from user
        // let onchange = Callback::from(|event:Event|{
        //     let target = event.target().unwrap();
        //     let value = target.unchecked_into::<HtmlInputElement>();
        // }
        // );
        let first_param: i64 = targeta.parse().unwrap();
        let second_param:i64 = targetb.parse().unwrap();
        match msg{
            Msg::Add => {
                self.result = first_param+second_param;
                return true;
            }
            Msg::Subtract => {
                self.result = first_param-second_param;
                return true;
            },
            Msg::Multiply => {
                self.result = first_param*second_param;
                return true;
            }
            Msg::Divide => {
                self.result = first_param - second_param;
                return true;
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // let on_input = ctx.link().callback(move |e: InputEvent| {
        //     let input_el: HtmlInputElement = e.target_unchecked_into();
        //     let val: i64 = input_el.value().parse().unwrap();
        // });
        html! {
            <div class="container">
                <button onclick={link.callback(|_| Msg::Add)}>{ "+" }</button>
                <button onclick={link.callback(|_| Msg::Subtract)}>{ "-" }</button>
                <button onclick={link.callback(|_| Msg::Multiply)}>{ "*" }</button>
                <button onclick={link.callback(|_| Msg::Divide)}>{ "/" }</button>
                <p>{self.result}</p>
            </div>
        }
    }
}

fn main(){
    yew::start_app::<Result>();
}