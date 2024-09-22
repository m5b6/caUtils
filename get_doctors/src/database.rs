use crate::models::DoctorProfile;
use colored::Colorize;
use sqlx::SqlitePool;
use std::error::Error;

pub async fn initialize_database(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS doctors (
        ppn INTEGER PRIMARY KEY,
        email TEXT,
        gender TEXT,
        fullName TEXT,
                name TEXT,
                lastName TEXT,
                image TEXT,
                tieneTelemedicina BOOLEAN,
                atencionClinica BOOLEAN
        );
        CREATE TABLE IF NOT EXISTS areas_medicas (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                doctor_ppn INTEGER,
                area_medica TEXT,
                FOREIGN KEY(doctor_ppn) REFERENCES doctors(ppn)
        );
        CREATE TABLE IF NOT EXISTS educacion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                doctor_ppn INTEGER,
                annoIngreso INTEGER,
                annoEgreso INTEGER,
                nombre TEXT,
                institucion TEXT,
                FOREIGN KEY(doctor_ppn) REFERENCES doctors(ppn)
        );
        CREATE TABLE IF NOT EXISTS especialidades (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                doctor_ppn INTEGER,
                especialidadId INTEGER,
                nombres TEXT,
                FOREIGN KEY(doctor_ppn) REFERENCES doctors(ppn)
        );
        -- Similarly, create other tables for areas_interes, que_trata, docencia, sub_especialidades, idiomas
        "#
        )
    .execute(pool)    
    .await?;
    Ok(())
}


pub async fn save_results_to_database(
    pool: &SqlitePool,
    profiles: &[DoctorProfile],
) -> Result<(), Box<dyn Error>> {
    let mut tx = pool.begin().await?;

    for profile in profiles {
        sqlx::query(
            r#"
            INSERT INTO doctors (
                ppn, email, gender, fullName, name, lastName, image,
                tieneTelemedicina, atencionClinica
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(ppn) DO UPDATE SET
                email=excluded.email,
                gender=excluded.gender,
                fullName=excluded.fullName,
                name=excluded.name,
                lastName=excluded.lastName,
                image=excluded.image,
                tieneTelemedicina=excluded.tieneTelemedicina,
                atencionClinica=excluded.atencionClinica;
            "#,
        )
        .bind(profile.ppn.map(|ppn| ppn as i64))
        .bind(&profile.email)
        .bind(&profile.gender)
        .bind(&profile.fullName)
        .bind(&profile.name)
        .bind(&profile.lastName)
        .bind(&profile.image)
        .bind(profile.tieneTelemedicina)
        .bind(profile.atencionClinica)
        .execute(&mut *tx)
        .await?;

        // Insert into areas_medicas table
        for area in &profile.areasMedicas {
            sqlx::query(
                r#"
                INSERT INTO areas_medicas (doctor_ppn, area_medica) VALUES (?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(area)
            .execute(&mut *tx)
            .await?;
        }

        for edu in &profile.educacion {
            sqlx::query(
                r#"
                INSERT INTO educacion (
                    doctor_ppn, annoIngreso, annoEgreso, nombre, institucion
                ) VALUES (?, ?, ?, ?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(edu.annoIngreso)
            .bind(edu.annoEgreso)
            .bind(&edu.nombre)
            .bind(&edu.institucion)
            .execute(&mut *tx)
            .await?;
        }

        for esp in &profile.especialidades {
            sqlx::query(
                r#"
                INSERT INTO especialidades (
                    doctor_ppn, especialidadId, nombres
                ) VALUES (?, ?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(esp.especialidadId)
            .bind(&esp.nombres)
            .execute(&mut *tx)
            .await?;
        }

        for idioma in &profile.idiomas {
            sqlx::query(r#"INSERT INTO idiomas (doctor_ppn, idioma) VALUES (?, ?);"#)
                .bind(profile.ppn.map(|ppn| ppn as i64))
                .bind(idioma)
                .execute(&mut *tx)
                .await?;
        }

        for area_inter in &profile.areasInteres {
            sqlx::query(
                r#"
                INSERT INTO areas_interes (doctor_ppn, area_interes) VALUES (?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(area_inter)
            .execute(&mut *tx)
            .await?;
        }

        for trata in &profile.queTrata {
            sqlx::query(
                r#"
                INSERT INTO que_trata (doctor_ppn, trata) VALUES (?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(trata)
            .execute(&mut *tx)
            .await?;
        }

        for doc in &profile.docencia {
            sqlx::query(
                r#"
                INSERT INTO docencia (doctor_ppn, docencia) VALUES (?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(doc)
            .execute(&mut *tx)
            .await?;
        }

        // Insert into sub_especialidades table
        for sub in &profile.subEspecialidades {
            sqlx::query(
                r#"
                INSERT INTO sub_especialidades (doctor_ppn, sub_especialidades) VALUES (?, ?);
                "#,
            )
            .bind(profile.ppn.map(|ppn| ppn as i64))
            .bind(sub)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    log::info!("{}", "Results saved to the database".on_green().bold());
    Ok(())
}
