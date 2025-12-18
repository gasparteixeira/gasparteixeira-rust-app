// UI Components Module - Single Responsibility Principle & Open/Closed Principle
// Reusable UI components separated by concern

use crate::api::User;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Props for UserForm component
#[derive(Properties, PartialEq, Clone)]
pub struct UserFormProps {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_editing: bool,
    pub on_name_change: Callback<String>,
    pub on_email_change: Callback<String>,
    pub on_password_change: Callback<String>,
    pub on_submit: Callback<()>,
    pub message: String,
}

#[function_component(UserForm)]
pub fn user_form(props: &UserFormProps) -> Html {
    let on_name_input = {
        let on_name_change = props.on_name_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_name_change.emit(input.value());
        })
    };

    let on_email_input = {
        let on_email_change = props.on_email_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_email_change.emit(input.value());
        })
    };

    let on_password_input = {
        let on_password_change = props.on_password_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_password_change.emit(input.value());
        })
    };

    let on_submit = {
        let callback = props.on_submit.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="mb-4">
            <input
                placeholder="Name"
                value={props.name.clone()}
                oninput={on_name_input}
                class="border rounded px-4 py-2 mr-2"
            />
            <input
                placeholder="Email"
                value={props.email.clone()}
                oninput={on_email_input}
                class="border rounded px-4 py-2 mr-2"
            />
            <input
                type="password"
                placeholder="Password"
                value={props.password.clone()}
                oninput={on_password_input}
                class="border rounded px-4 py-2 mr-2"
            />
            <button
                onclick={on_submit}
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
                { if props.is_editing { "Update User" } else { "Create User" } }
            </button>
            if !props.message.is_empty() {
                <p class="text-green-500 mt-2">{ &props.message }</p>
            }
        </div>
    }
}

// Props for UserList component
#[derive(Properties, PartialEq)]
pub struct UserListProps {
    pub users: Vec<User>,
    pub on_delete: Callback<i32>,
    pub on_edit: Callback<i32>,
}

#[function_component(UserList)]
pub fn user_list(props: &UserListProps) -> Html {
    html! {
        <div class="p-6">
            <h2 class="text-2xl font-bold text-gray-700 mb-2">{ "User List" }</h2>
            <div class="grid grid-cols-[50px_1fr_1fr_100px_100px] gap-4 px-4 py-2 bg-gray-100 font-bold text-gray-700 border-b">
              <div>{ "ID" }</div>
              <div>{ "Name" }</div>
              <div>{ "Email" }</div>
              <div>{ "" }</div>
              <div>{ "" }</div>
            </div>
            <ul class="divide-y divide-gray-200">
                { for props.users.iter().map(|user| {
                    html! { <UserListItem key={user.id} user={user.clone()} on_delete={props.on_delete.clone()} on_edit={props.on_edit.clone()} /> }
                })}
            </ul>
        </div>
    }
}

// Props for UserListItem component
#[derive(Properties, PartialEq)]
pub struct UserListItemProps {
    pub user: User,
    pub on_delete: Callback<i32>,
    pub on_edit: Callback<i32>,
}

#[function_component(UserListItem)]
pub fn user_list_item(props: &UserListItemProps) -> Html {
    let user_id = props.user.id;
    let on_delete = {
        let callback = props.on_delete.clone();
        Callback::from(move |_| callback.emit(user_id))
    };

    let on_edit = {
        let callback = props.on_edit.clone();
        Callback::from(move |_| callback.emit(user_id))
    };

    html! {
        <li class="grid grid-cols-[50px_1fr_1fr_100px_100px] gap-4 px-4 py-2 hover:bg-gray-50 items-center">
            <span class="font-medium text-gray-900">
                { format!("{}", props.user.id) }
            </span>
            <span class="font-medium text-gray-900">
                { format!("{}", props.user.name) }
            </span> <span class="font-medium text-gray-900">
                { format!("{}", props.user.email) }
            </span>
            <button
                onclick={on_delete}
                class=" bg-red-500 hover:bg-red-700 text-white py-1 px-2 rounded"
            >
                { "Delete" }
            </button>
            <button
                onclick={on_edit}
                class=" bg-yellow-500 hover:bg-yellow-700 text-white  py-1 px-2 rounded"
            >
                { "Edit" }
            </button>
        </li>
    }
}

// Props for Button component
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = {
        let callback = props.onclick.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <button
            onclick={onclick}
            class={props.class.clone()}
        >
            { &props.text }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_form_props_creation() {
        let props1 = UserFormProps {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
            is_editing: false,
            on_name_change: Callback::noop(),
            on_email_change: Callback::noop(),
            on_password_change: Callback::noop(),
            on_submit: Callback::noop(),
            message: "Success".to_string(),
        };

        assert_eq!(props1.name, "John");
        assert_eq!(props1.email, "john@example.com");
        assert_eq!(props1.password, "password123");
        assert!(!props1.is_editing);
    }

    #[test]
    fn test_user_list_props_creation() {
        let users = vec![User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        }];

        let props1 = UserListProps {
            users: users.clone(),
            on_delete: Callback::noop(),
            on_edit: Callback::noop(),
        };

        assert_eq!(props1.users.len(), 1);
    }

    #[test]
    fn test_user_list_item_props() {
        let user = User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        };

        let props = UserListItemProps {
            user: user.clone(),
            on_delete: Callback::noop(),
            on_edit: Callback::noop(),
        };

        assert_eq!(props.user.id, 1);
        assert_eq!(props.user.name, "John");
        assert_eq!(props.user.email, "john@example.com");
    }

    #[test]
    fn test_button_props() {
        let props = ButtonProps {
            text: "Click Me".to_string(),
            onclick: Callback::noop(),
            class: "btn-primary".to_string(),
        };

        assert_eq!(props.text, "Click Me");
        assert_eq!(props.class, "btn-primary");
    }

    #[test]
    fn test_button_props_default_class() {
        let props = ButtonProps {
            text: "Click Me".to_string(),
            onclick: Callback::noop(),
            class: String::new(),
        };

        assert_eq!(props.class, "");
    }
}
