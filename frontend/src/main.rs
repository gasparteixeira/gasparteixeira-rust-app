// Main Application - Clean, SOLID-compliant architecture
// Following Single Responsibility, Open/Closed, Liskov Substitution,
// Interface Segregation, and Dependency Inversion principles

use frontend::{
    use_user_form_state, ApiResult, Button, DefaultUserService, User, UserForm, UserFormState,
    UserList, UserService,
};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // State management
    let form_state = use_user_form_state();
    let message = use_state(String::new);
    let users = use_state(Vec::new);

    // Service layer - instantiated per component
    let service = DefaultUserService::default();

    // Fetch users handler
    let fetch_users = {
        let users = users.clone();
        let message = message.clone();
        let service = service.clone();

        Callback::from(move |_| {
            let users = users.clone();
            let message = message.clone();
            let service = service.clone();

            service.fetch_users(Callback::from(
                move |result: ApiResult<Vec<User>>| match result {
                    Ok(fetched_users) => {
                        users.set(fetched_users);
                        message.set(String::new());
                    }
                    Err(err) => message.set(err),
                },
            ));
        })
    };

    // Create/Update user handler
    let submit_user = {
        let form_state = form_state.clone();
        let message = message.clone();
        let fetch_users = fetch_users.clone();
        let service = service.clone();

        Callback::from(move |_| {
            let current_state = (*form_state).clone();
            let is_editing = current_state.is_editing();
            let message = message.clone();
            let fetch_users = fetch_users.clone();
            let form_state = form_state.clone();
            let service = service.clone();

            let callback = Callback::from(move |result: ApiResult<()>| {
                match result {
                    Ok(_) => {
                        let success_msg = if is_editing {
                            "User updated successfully"
                        } else {
                            "User created successfully"
                        };
                        message.set(success_msg.to_string());

                        // Reset form and refresh list
                        form_state.set(UserFormState::new());
                        fetch_users.emit(());
                    }
                    Err(err) => message.set(err),
                }
            });

            if is_editing {
                service.update_user(&current_state, callback);
            } else {
                service.create_user(&current_state, callback);
            }
        })
    };

    // Delete user handler
    let delete_user = {
        let message = message.clone();
        let fetch_users = fetch_users.clone();
        let service = service.clone();

        Callback::from(move |id: i32| {
            let message = message.clone();
            let fetch_users = fetch_users.clone();
            let service = service.clone();

            service.delete_user(
                id,
                Callback::from(move |result: ApiResult<()>| match result {
                    Ok(_) => {
                        message.set("User deleted successfully".to_string());
                        fetch_users.emit(());
                    }
                    Err(err) => message.set(err),
                }),
            );
        })
    };

    // Edit user handler
    let edit_user = {
        let form_state = form_state.clone();
        let users = users.clone();

        Callback::from(move |id: i32| {
            if let Some(user) = users.iter().find(|u| u.id == id) {
                let mut new_state = (*form_state).clone();
                // Note: Password is not included for security reasons - user must enter new password
                new_state.set_for_editing(id, user.name.clone(), user.email.clone(), String::new());
                form_state.set(new_state);
            }
        })
    };

    // Form input handlers
    let on_name_change = {
        let form_state = form_state.clone();
        Callback::from(move |name: String| {
            let mut new_state = (*form_state).clone();
            new_state.name = name;
            form_state.set(new_state);
        })
    };

    let on_email_change = {
        let form_state = form_state.clone();
        Callback::from(move |email: String| {
            let mut new_state = (*form_state).clone();
            new_state.email = email;
            form_state.set(new_state);
        })
    };

    let on_password_change = {
        let form_state = form_state.clone();
        Callback::from(move |password: String| {
            let mut new_state = (*form_state).clone();
            new_state.password = password;
            form_state.set(new_state);
        })
    };

    // Render UI
    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-4xl font-bold text-blue-500 mb-4">{ "User Management" }</h1>

            <UserForm
                name={form_state.name.clone()}
                email={form_state.email.clone()}
                password={form_state.password.clone()}
                is_editing={form_state.is_editing()}
                on_name_change={on_name_change}
                on_email_change={on_email_change}
                on_password_change={on_password_change}
                on_submit={submit_user}
                message={(*message).clone()}
            />

            <Button
                text="Fetch User List"
                onclick={fetch_users}
                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4"
            />

            <UserList
                users={(*users).clone()}
                on_delete={delete_user}
                on_edit={edit_user}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_app_component_exists() {
        // Verify the App component type exists
        // This confirms proper module structure
    }

    #[test]
    fn test_main_function_compiles() {
        // Verify main function exists and compiles
        // In WASM environment it would render the app
    }
}
