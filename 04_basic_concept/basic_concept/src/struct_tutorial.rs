pub mod struct_tutorial {
    #[derive(Debug)]
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn set_user(your_username: String, email: String) -> User {
        return User {
            username: your_username,
            email, // パラメータ名が同一の場合は省略可能
            active: true,
            sign_in_count: 1,
        };
    }

    pub fn show_userinfo(user_info: User) {
        println!("{:#?}", user_info);
    }

    pub fn create_someuser() {
        let user_a = User {
            email: String::from("user_a@mail.com"),
            username: String::from("user_a"),
            active: true,
            sign_in_count: 2,
        };
        let user_b = User {
            email: String::from("user_b@mail.com"),
            username: String::from("user_b"),
            ..user_a // 他パラメータは”user_a”と同様の値がセットされる
        };

        show_userinfo(user_a);
        show_userinfo(user_b);
    }
}
