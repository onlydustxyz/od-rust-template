diesel::table! {
	users (id) {
		id -> Varchar,
		name -> Varchar,
	}
}

diesel::table! {
	organizations (id) {
		id -> Varchar,
		name -> Varchar,
		external_id -> VarChar,
	}
}
