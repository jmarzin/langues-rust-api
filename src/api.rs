use rocket_contrib::json::Json;
use rocket::http::RawStr;
use crate::MyPgDatabase;

#[get("/api")]
pub fn api() -> String {
    "The api works !".to_string()
}

#[get("/api/v1/langues")]
pub fn v1_langues(conn: MyPgDatabase) -> Result<Json<Vec<String>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<String> = Vec::new();
    for row in &conn.query("SELECT language_id FROM languages", &[])? {
        liste.push(row.get("language_id"));
    };
    Ok(Json(liste))
}

#[get("/<langue>/api/v1/date_themes")]
pub fn v1_date_themes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from themes where language_id= $1 and supp = 'f'", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    };
    Ok("".to_string())
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v1/themes")]
pub fn v1_themes(langue: &RawStr, conn: MyPgDatabase) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    for row in &conn.query("select id, number, in_language from themes where language_id = $1 and supp = 'f'", &[&langue.to_string()])? {
        let id: i32 = row.get(0);
        let number: i32 = row.get(1);
        let in_language: String = row.get(2);
        let res: Vec<String> = vec![id.to_string(), number.to_string(), in_language];
        liste.push(res);
    };
    Ok(Json(liste))
}

fn date_max_v2_themes(langue: &RawStr, conn: &MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from themes where language_id= $1", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    }
    Ok("".to_string())
}

#[get("/<langue>/api/v2/date_themes")]
pub fn v2_date_themes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    date_max_v2_themes(langue, &conn)
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v2/themes?<date>")]
pub fn v2_themes(langue: &RawStr, date: &RawStr, conn: MyPgDatabase) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    let date_au_plus_tot = date.to_string().replace("%20", " ");
    for row in &conn.query("select id, number, in_language, supp from themes where language_id = $1 and last_update > $2", &[&langue.to_string(), &date_au_plus_tot])? {
        let id: i32 = row.get(0);
        let number: i32 = row.get(1);
        let in_language: String = row.get(2);
        let supp: String = row.get(3);
        let res: Vec<String> = vec![id.to_string(), number.to_string(), in_language, supp];
        liste.push(res);
    };
    Ok(Json(liste))
}

#[get("/<langue>/api/v1/date_mots")]
pub fn v1_date_mots(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from words where language_id= $1 and supp = 'f'", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    };
    Ok("".to_string())
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v3/mots")]
pub fn v3_mots(langue: &RawStr, conn: MyPgDatabase) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    for row in &conn.query(
            "select id, theme_id, in_french, sort_word, in_language, coalesce(language_level,'') as language_l, coalesce(pronunciation, '') as pronounce from words where language_id = $1 and supp = 'f'",
            &[&langue.to_string()])? {
        let id: i32 = row.get(0);
        let theme_id: i32 = row.get(1);
        let in_french: String = row.get(2);
        let sort_word: String = row.get(3);
        let in_language: String = row.get(4);
        let language_level: String = row.get(5);
        let pronunciation: String = row.get(6);
        let res: Vec<String> = vec![id.to_string(), theme_id.to_string(), in_french, sort_word, in_language, language_level, pronunciation];
        liste.push(res);
    };
    Ok(Json(liste))
}

fn date_max_v2_mots(langue: &RawStr, conn: &MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from words where language_id= $1", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    }
    Ok("".to_string())
}

#[get("/<langue>/api/v2/date_mots")]
pub fn v2_date_mots(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    date_max_v2_mots(langue, &conn)
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v4/mots?<date>")]
pub fn v4_mots(langue: &RawStr, conn: MyPgDatabase, date: &RawStr) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    let date_au_plus_tot = date.to_string().replace("%20", " ");
    for row in &conn.query(
        "select id, theme_id, in_french, sort_word, in_language, coalesce(language_level,'') as language_l, coalesce(pronunciation, '') as pronounce, supp from words where language_id = $1 and last_update > $2",
        &[&langue.to_string(), &date_au_plus_tot])? {
        let id: i32 = row.get(0);
        let theme_id: i32 = row.get(1);
        let in_french: String = row.get(2);
        let sort_word: String = row.get(3);
        let in_language: String = row.get(4);
        let language_level: String = row.get(5);
        let pronunciation: String = row.get(6);
        let supp: String = row.get(7);
        let res: Vec<String> = vec![id.to_string(), theme_id.to_string(), in_french, sort_word,
                                    in_language, language_level, pronunciation, supp];
        liste.push(res);
    };
    Ok(Json(liste))
}

#[get("/<langue>/api/v1/date_verbes")]
pub fn v1_date_verbes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from verbs where language_id= $1 and supp = 'f'", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    };
    Ok("".to_string())
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v1/verbes")]
pub fn v1_verbes(langue: &RawStr, conn: MyPgDatabase) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    for row in &conn.query("select id, in_language from verbs where language_id = $1 and supp = 'f'", &[&langue.to_string()])? {
        let id: i32 = row.get(0);
        let in_language: String = row.get(1);
        let res: Vec<String> = vec![id.to_string(), in_language];
        liste.push(res);
    };
    Ok(Json(liste))
}

fn date_max_v2_verbes(langue: &RawStr, conn: &MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from verbs where language_id= $1", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    }
    Ok("".to_string())
}

#[get("/<langue>/api/v2/date_verbes")]
pub fn v2_date_verbes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    date_max_v2_verbes(langue, &conn)
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v2/verbes?<date>")]
pub fn v2_verbes(langue: &RawStr, conn: MyPgDatabase, date: &RawStr) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    let date_au_plus_tot = date.to_string().replace("%20", " ");
    for row in &conn.query("select id, in_language, supp from verbs where language_id = $1 and last_update > $2",
                           &[&langue.to_string(), &date_au_plus_tot])? {
        let id: i32 = row.get(0);
        let in_language: String = row.get(1);
        let supp: String = row.get(2);
        let res: Vec<String> = vec![id.to_string(), in_language, supp];
        liste.push(res);
    };
    Ok(Json(liste))
}

#[get("/<langue>/api/v1/date_formes")]
pub fn v1_date_formes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from forms where language_id= $1 and supp = 'f'", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    };
    Ok("".to_string())
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v1/formes")]
pub fn v1_formes(langue: &RawStr, conn: MyPgDatabase) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    for row in &conn.query(
        "select t1.id, t1.verb_id, t2.number, t1.in_language from forms t1, formstypes t2 where t1.language_id = $1 and t1.form_type_id = t2.id and t1.supp = 'f'",
        &[&langue.to_string()])? {
        let id: i32 = row.get(0);
        let verb_id: i32 = row.get(1);
        let form_type_number: i32 = row.get(2);
        let in_language: String = row.get(3);
        let res: Vec<String> = vec![id.to_string(), verb_id.to_string(), form_type_number.to_string(), in_language];
        liste.push(res);
    };
    Ok(Json(liste))
}

fn date_max_v2_formes(langue: &RawStr, conn: &MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from forms where language_id= $1", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    }
    Ok("".to_string())
}

#[get("/<langue>/api/v2/date_formes")]
pub fn v2_date_formes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    date_max_v2_formes(langue, &conn)
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v2/formes?<date>")]
pub fn v2_formes(langue: &RawStr, conn: MyPgDatabase, date: &RawStr) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    let date_au_plus_tot = date.to_string().replace("%20", " ");
    for row in &conn.query(
        "select t1.id, t1.verb_id, t2.number, t1.in_language, t1.supp from forms t1, formstypes t2 where t1.language_id = $1 and t1.form_type_id = t2.id and t1.last_update > $2",
        &[&langue.to_string(), &date_au_plus_tot])? {
        let id: i32 = row.get(0);
        let verb_id: i32 = row.get(1);
        let form_type_number: i32 = row.get(2);
        let in_language: String = row.get(3);
        let supp: String = row.get(4);
        let res: Vec<String> = vec![id.to_string(), verb_id.to_string(), form_type_number.to_string(), in_language, supp];
        liste.push(res);
    };
    Ok(Json(liste))
}

fn date_max_v1_formestypes(langue: &RawStr, conn: &MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    for row in &conn.query("select COALESCE(MAX(last_update), '') from formstypes where language_id= $1", &[&(langue.to_string())])? {
        let res: String = row.get(0);
        return Ok(res)
    }
    Ok("".to_string())
}

#[get("/<langue>/api/v1/date_formestypes")]
pub fn v1_date_formestypes(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    date_max_v1_formestypes(langue, &conn)
}

//SELECT id, number, in_language FROM themes WHERE language_id = 'it';

#[get("/<langue>/api/v1/formestypes?<date>")]
pub fn v1_formestypes(langue: &RawStr, conn: MyPgDatabase, date: &RawStr) -> Result<Json<Vec<Vec<String>>>, Box<dyn std::error::Error + 'static>> {
    let mut liste: Vec<Vec<String>> = Vec::new();
    let date_au_plus_tot = date.to_string().replace("%20", " ");
    for row in &conn.query(
        "select id, in_french, number, supp from formstypes where language_id = $1 and last_update > $2",
        &[&langue.to_string(), &date_au_plus_tot])? {
        let id: i32 = row.get(0);
        let in_french: String = row.get(1);
        let number: i32 = row.get(2);
        let supp: String = row.get(3);
        let res: Vec<String> = vec![id.to_string(), in_french, number.to_string(), supp];
        liste.push(res);
    };
    Ok(Json(liste))
}

#[get("/<langue>/api/v1/date_maj")]
pub fn v1_date_maj(langue: &RawStr, conn: MyPgDatabase) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let d1 = date_max_v1_formestypes(&langue, &conn)?;
    println!("{}", d1);
    let d2 = date_max_v2_formes(&langue, &conn)?;
    println!("{}", d2);
    let mut max = if d1 > d2 {d1} else {d2};
    let d3 = date_max_v2_verbes(&langue, &conn)?;
    println!("{}", d3);
    max = if d3 > max {d3} else {max};
    let d4 = date_max_v2_mots(&langue, &conn)?;
    println!("{}", d4);
    max = if d4 > max {d4} else {max};
    let d5 = date_max_v2_themes(&langue, &conn)?;
    println!("{}", d5);
    max = if d5 > max {d5} else {max};
    Ok(max.to_string())
}
