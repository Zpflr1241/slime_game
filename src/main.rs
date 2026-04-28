use std:: io;
fn get_input() -> i32 {
    let mut input = String:: new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let input: i32 = match input.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    };
    input
}
struct Slime {
        hunger: i32,
        clean: i32,
        happy: i32,
        hung: i32,
    }
fn play_slime(slime: &Slime ) -> bool {
    
    if slime.hunger <= 0 {
        println!("(X _ x) 슬라임이 너무 배가 고파서 죽었습니다... 게임 오버");
        return false;
    } else if slime.clean <= 0 {
        println!("방이 너무 더러워 병에 걸려 죽었습니다... 게임 오버");
        return false;
    } else if slime.hunger > 10 {
        println!("슬라임이 밥을 너무 많이 먹어서 배가 터져버렸습니다... 게임 오버");
        return false;
    } else if slime.happy <= 0 {
        println!("슬라임이 우울하여 죽었습니다... 게임 오버");
        return false;
    } else {
        return true;
    }
}

fn status(slime: &Slime) {
    println!("\n===============================");
    println!("현재 포만감: {} | 현재 청결도 {} | 현재 행복도 {}",slime.hunger, slime.clean, slime.happy);
    println!("무엇을 할까요?");
    println!("1. 밥 주기 (청결도 -2, 포만감 +4)");
    println!("2. 놀아 주기 (포만감 -2, 행복도 +3, 청결도 -2)");
    println!("3. 잠재우기 (게임 종료)");
    println!("4. 방 청소하기 포만감 -2, 행복도 -1 ,청결도 초기화");
    println!("===============================");
}
fn main() {

    let mut my_slime = Slime {
        hunger: 5,
        clean: 10,
        happy: 5,
        hung: 0,
    };
    println!("짜잔 귀여운 슬라임이 태어났습니다 열심히 키워 보세요.");
    loop {
            if play_slime(&my_slime) == false {
                break;
            }
            status(&my_slime);
            let action = get_input();
            
            match action {
                1 => {
                    my_slime.hunger = my_slime.hunger + 4;
                    my_slime.clean = my_slime.clean - 2;
                    my_slime.hung = my_slime.hung + 1;
                        
                        if my_slime.hung % 2 == 0 {
                        my_slime.happy = my_slime.happy -1;
                        println!("( 냠냠 ) 밥을 먹었지만 계속 밥만 먹어서 지루합니다.");
                    } else {
                    println!("( 냠냠 ) 밥을 먹었습니다.");
                }
            }
                2 => {
                    my_slime.hunger = my_slime.hunger - 2;
                    my_slime.happy = my_slime.happy + 3;
                    my_slime.clean = my_slime.clean -2;
                    println!("( 히히히 ) 재밌게 놀았습니다.");
                }
                3 => {
                    println!("잘 자 (게임을 종료합니다.)");
                    break;
                }
                4 => {
                    my_slime.hunger = my_slime.hunger -2;
                    my_slime.clean = 10;
                    my_slime.happy = my_slime.happy -2;
                    println!("( 재밌는 방 청소 )방을 청소합니다.");
                }
                _ => println!("없는 선택지 입니다."),
            }
        }
    }

    

