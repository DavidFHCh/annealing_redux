extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;

struct Conn {
    id1: u16,
    id2: u16,
    dist: f64,
}

pub fn make_cities() -> Result<Vec<Vec<f64>>, rusqlite::Error> {
    let db_path = Path::new("hoc.db");
    let conn = Connection::open(db_path).unwrap();
    let total_cities: u32 = conn.query_row("select count(id) from cities", &[], |row| row.get(0))
        .expect("Error al preparar conexión para obtener total de ciudades");
    let t_c = (total_cities + 1) as usize;
    let mut cities: Vec<Vec<f64>> = vec![vec![-1.0; t_c]; t_c];
    let mut stmt = conn.prepare("select id_city_1, id_city_2, distance from connections")
        .expect(
            "Error al preparar conexión para obtener distancias entre ciudades.",
        );
    let conn_iter = stmt.query_map(&[], |row| {
        Conn {
            id1: row.get(0),
            id2: row.get(1),
            dist: row.get(2),
        }
    }).unwrap();

    for conn in conn_iter {
        let conn_u = conn.unwrap();
        let id1 = conn_u.id1 as usize;
        let id2 = conn_u.id2 as usize;
        let dist = conn_u.dist;
        cities[id1][id2] = dist;
        cities[id2][id1] = dist;
    }

    Ok(cities)
}
