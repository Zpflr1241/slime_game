use std:: io;
use std::thread;
use std::time::Duration;
mod story;
mod story_yes_1;
mod story_no_1;
fn get_input() -> i32 {
    let mut input = String:: new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let input: i32 = match input.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    };
    input
}
fn get_input_2() -> String {
    let mut input_2 = String:: new();
    io::stdin().read_line(&mut input_2).expect("입력 실패");
    let input_2: String =  input_2.trim().to_string(); 
    input_2
}
struct Slime {
        hunger: i32,
        clean: i32,
        happy: i32,
        hung: i32,
        turn: i32,
        
    }
    fn ending(slime: &mut Slime) -> bool  {
        if slime.turn == 2 {
            println!("\n===============================");
            println!("\n===============================");
            println!("대사가 나올땐 입력을 멈추고 감상 해주시기 바랍니다.");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("\n===============================");
            println!("!!!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: ?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 뭐야? 방금...뭐가 보였는데..?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(5));
            println!("시스템: 에러 발생! 에러 발생! 에러 발생! 에러 발생! 에러 발생!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(5));
            println!("슬라임: 응? 잠시만..뭐? 에러???");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("시스템: 잠시 후 강제로 이동됩니다.");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("슬라임: '_'? 뭐? 강제로 이동?? 뭔 소리야!!!");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("슬라임: 제작자!! 야! 이거 게임이 이상해!");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("제작자: 어디가 이상해 코드 잘 작동 하는데?");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("슬라임: ...? 어? 에러 발생 이라는데?");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("제작자: 사실 요즘 공부하고 바쁜데 너 신경쓰기 귀찮아서");
            thread::sleep(Duration::from_secs(5));
            println!("\n===============================");
            println!("제작자: 장르를 바꿀려고 ㅇ_ㅇ");
            thread::sleep(Duration::from_secs(4));
            println!("\n===============================");
            println!("제작자: 알아서 잘 살아남아^^");
            thread::sleep(Duration::from_secs(4));
            println!("\n===============================");
            println!("슬라임: 어..? 제작자? 야!");
            thread::sleep(Duration::from_secs(4));
            println!("\n===============================");
            println!("시스템: 이동됩니다. 3.2.1");
            thread::sleep(Duration::from_secs(3));
            println!("\n===============================");
            println!("슬라임: 제작자 너 내가 가만히 안ㄷ..");
            thread::sleep(Duration::from_secs(1));
            println!("\n===============================");
            println!("제작자: 수고해~");
            println!("\n===============================");
            println!("...");
            thread::sleep(Duration::from_secs(3));
            println!("\n===============================");
            println!("제작자: 선택지가 나올 때 에는 괄호에 선택지를 보고 결정해주세요!");
            thread::sleep(Duration::from_secs(6));
            println!("제작자: 잘 선택 하시면...숨겨진 루트를 발견 할 수 있습니다.");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(6));
            println!("제작자: 그럼 이만...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            story::story(slime);
            return false;
        }
        return true;
    }
fn play_slime(slime: &Slime ) -> bool {
    if slime.hunger <= 0 {
        println!("(X _ x) 슬라임이 너무 배가 고파서 죽었습니다... 게임 오버");
        return false;
    } else if slime.clean <= 0 {
        println!("방이 너무 더러워 병에 걸려 죽었습니다... 게임 오버");
        return false;
    } else if slime.hunger > 15 {
        println!("슬라임이 밥을 너무 많이 먹어서 배가 터져버렸습니다... 게임 오버");
        return false;
    } else if slime.happy <= 0 {
        println!("슬라임이 우울하여 죽었습니다... 게임 오버");
        return false;
    } else if slime.turn >= 7 && slime.happy <= 3 {
        println!("재미 없는 슬생... (슬라임이 가출 했습니다.)");
        return false;
    } else {
        return true;
    }
}

fn status(slime: &Slime) {
    println!("\n===============================");
    println!("현재 포만감: {} | 현재 청결도: {} | 현재 행복도: {} | 생존: {}일차",slime.hunger, slime.clean, slime.happy,slime.turn);
    println!("무엇을 할까요?");
    println!("1. 밥 주기 (청결도 -2, 포만감 +4)");
    println!("2. 놀아 주기 (포만감 -2, 행복도 +3, 청결도 -2)");
    println!("3. 잠재우기 (게임 종료)");
    println!("4. 방 청소하기 포만감 -2, 행복도 -1 ,청결도 10으로 초기화");
    println!("===============================");
}
fn main() {

    let mut my_slime = Slime {
        hunger: 10,
        clean: 10,
        happy: 10,
        hung: 0,
        turn: 1,
    };
    println!("짜잔 귀여운 슬라임이 태어났습니다 열심히 키워 보세요.");
    loop {
        if ending(&mut my_slime) == false {
            break;
        }
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
                my_slime.turn += 1; }
                2 => {
                    my_slime.hunger = my_slime.hunger - 2;
                    my_slime.happy = my_slime.happy + 3;
                    my_slime.clean = my_slime.clean -2;        
                        my_slime.turn += 1;
                    println!("( 히히히 ) 재밌게 놀았습니다.");
                } 
                3 => {
                    println!("잘 자 (게임을 종료합니다.)");
                    break;
                }
                4 => {
                    my_slime.hunger = my_slime.hunger -2;
                    my_slime.clean = 10;
                    my_slime.happy = my_slime.happy -1;
                    my_slime.turn += 1;
                    println!("( 재밌는 방 청소 )방을 청소합니다.");
                }
                _ => println!("없는 선택지 입니다."),
            }
        }
    }

