#![allow(non_snake_case)]
use macroquad::prelude::*;
use std::env;
use ::rand::Rng;
use std::fs;
use std::thread;
use std::time::Duration;
use std::io;
mod ka;
const TRATE: i32 = 60;
const TICK: f64 = 1.0 / 60.0;
const G: f64 = 982.5;
#[macroquad::main("pink")]
async fn main() {
    let mut ticktimer: i32 = 0;
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Input Err");
    let mut PState = ka::Stats {
        mHP: 5,
        cHP: 5,
        Dmg: 1,
        NaC: name.trim().to_string(),
        aRg: 250.0,
        wVl: 250.0,
        rVl: 500.0,
        aCd: 0.5,
        Grd: false,
        poX: 200.0,
        poY: 170.0,
        cVX: 0.0,
        cVY: 0.0,
        isY: true,
    };
    let mut guy = enemy();
    loop {
        clear_background(BLACK);
        draw_rectangle(0.0, 200.0, 1000.0, 120.0, PINK);
        if is_key_down(KeyCode::Up) {
            if PState.Grd == false {}
            else {
            PState.cVY -= 400.0;
            PState.Grd = false;
            };
        };
        if is_key_down(KeyCode::Right) {
        PState.cVX += PState.wVl * TICK;
        }
        else if is_key_down(KeyCode::Left) {
        PState.cVX -= PState.wVl * TICK;
        }
        else {
            if PState.cVX > 0.0 { 
                PState.cVX -= 200.0 * TICK;
                }
            else if PState.cVX < 0.0 {
                PState.cVX += 200.0 * TICK;
            };
        };
        if is_key_down(KeyCode::Escape) { break };
        if PState.aCd > 0.0 {
            PState.aCd -= TICK;
            if PState.aCd < 0.0 { PState.aCd = 0.0; }
        };
        if PState.Grd {} 
        else {
            PState.cVY += G * TICK;
        };
        if PState.poY > 170.0 && PState.poY < 320.0 {
            PState.Grd = true;
            PState.cVY = 0.0;
            PState.poY = 170.0;
        };
        ticktimer += 1;
        if ticktimer == 240 {
            guy.poX = ::rand::thread_rng().gen_range(0..860) as f64;
            guy.isY = true;
            ticktimer -=240
        };
        if guy.isY && guy.poY < 170.0 {
        guy.poY += 1.0;
        draw_rectangle(guy.poX as f32, guy.poY as f32, 30.0, 30.0, PURPLE);
        }
        else if guy.poY >= 170.0 {
            guy.isY = false;
            guy.poY = -30.0;
        };
        PState.poY += PState.cVY * TICK;
        PState.poX += PState.cVX * TICK;
        if guy.isY {
            let cox = (PState.poX + 30.0 > guy.poX) && (PState.poX < guy.poX + 30.0);
            let coy = (PState.poY + 30.0 > guy.poY) && (PState.poY < guy.poY + 30.0);
            if cox && coy && PState.aCd == 0.0 {
                PState.cHP -= guy.Dmg;
                PState.aCd = 1.0;
            }
        };  
        if PState.cHP == 0 {
            break;
        };
        draw_rectangle(PState.poX as f32, PState.poY as f32, 30.0, 30.0, GREEN);
        let mut hp = format!("you have {} hp", PState.cHP);
        draw_text(&hp, 10.0, 10.0, 15.0, GREEN); 
        next_frame().await;
    };
}
fn enemy() -> ka::Stats {
    let mut Vile = ka::Stats {
        poX: ::rand::thread_rng().gen_range(0..860) as f64,
        poY: -30.0,
        cVY: 0.0,
        isY: false,
        Dmg: 1,
        cHP: 1,
        mHP: 1,
        Grd: false,
        aRg: 0.0,
        wVl: 0.0,
        rVl: 0.0,
        cVX: 0.0,
        aCd: 0.0,
        NaC: String::from("guy"),
    };
    Vile
}


