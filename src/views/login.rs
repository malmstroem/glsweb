use crate::routes::USR;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
//use dsrv::server::functions::login;
#[cfg(feature = "server")]
use dsrv::auth::{Session, User};

#[component]
pub fn Login() -> Element {
    let mut status = use_signal(|| String::from(""));

    let onsubmit = move |evt: FormEvent| async move {
        let username: String = evt.values()["username"][0].clone();
        let password: String = evt.values()["password"][0].clone();
        let resp = login(username.clone(), password).await;
        info!("Response: {resp:?}");

        match resp {
            Ok(data) => match data.as_ref() {
                "login successful" => {
                    *USR.write() = username.clone();
                    let nav = navigator();
                    nav.replace(Route::Home {});
                }
                "user not found" => *status.write() = data.to_string(),
                "incorrect password" => *status.write() = data.to_string(),
                _ => *status.write() = format!("unknown response: {data}"),
            },
            Err(err) => {
                *status.write() = format!("Login failed {err:?}");
            }
        }
    };

    rsx! {
        div { class: "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8",
            div { class: "sm:mx-auto sm:w-full sm:max-w-sm",
                p { class: "text-center text-lg font-bold leading-9 tracking-tight text-gray-900",
                    {status}
                }
            }
            div { class: "sm:mx-auto sm:w-full sm:max-w-sm",
                h2 { class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900",
                    "Sign in to your account"
                }
            }

            div { class: "mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                form { class: "space-y-6", onsubmit,
                    div {
                        label {
                            r#for: "username",
                            class: "block text-sm font-medium leading-6 text-gray-900",
                            "Username"
                        }
                        div { class: "mt-2",
                            input {
                                id: "username",
                                name: "username",
                                autocomplete: "username",
                                required: "required",
                                class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                            }
                        }

                        div {
                            div { class: "flex items-center justify-between",
                                label {
                                    r#for: "password",
                                    class: "block text-sm font-medium leading-6 text-gray-900",
                                    "Password"
                                }
                            }
                            div { class: "mt-2",
                                input {
                                    id: "password",
                                    name: "password",
                                    r#type: "password",
                                    autocomplete: "current-password",
                                    required: "required",
                                    class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                                }
                            }

                            div {
                                button {
                                    r#type: "submit",
                                    class: "flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                    "Sign in"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "server")]
#[derive(sqlx::FromRow, Debug)]
pub struct DbUser {
    id: i32,
    username: String,
    password: String,
}

#[server(ServerLogin)]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let dbc = session.1;
    let user: DbUser = match sqlx::query_as::<_, DbUser>(
        "SELECT id,username,password FROM users where username = $1",
    )
    .bind(username)
    .fetch_one(dbc.as_ref())
    .await
    {
        Ok(user) => user,
        Err(err) => return Ok("user not found".into()), // FIXME: replace with error
    };
    info!("{user:?}");
    if user.password != password {
        return Ok("incorrect password".into()); // FIXME: replace with error
    }
    info!("Match");
    auth.login_user(user.id as i64);
    Ok("login successful".into())
}
