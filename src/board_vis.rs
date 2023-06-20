use chess::*;
use image::*;

pub fn board_to_image(board: &Board) -> Vec<u8> {
    let cburnett = image::open("cburnett.png").unwrap(); // TODO: Hide behind lazy_static thing

    let king_white   = imageops::crop_imm(&cburnett, 256*0,0, 256,256).to_image();
    let queen_white  = imageops::crop_imm(&cburnett, 256*1,0, 256,256).to_image();
    let bishop_white = imageops::crop_imm(&cburnett, 256*2,0, 256,256).to_image();
    let knight_white = imageops::crop_imm(&cburnett, 256*3,0, 256,256).to_image();
    let rook_white   = imageops::crop_imm(&cburnett, 256*4,0, 256,256).to_image();
    let pawn_white   = imageops::crop_imm(&cburnett, 256*5,0, 256,256).to_image();

    let king_black   = imageops::crop_imm(&cburnett, 256*0,256, 256,256).to_image();
    let queen_black  = imageops::crop_imm(&cburnett, 256*1,256, 256,256).to_image();
    let bishop_black = imageops::crop_imm(&cburnett, 256*2,256, 256,256).to_image();
    let knight_black = imageops::crop_imm(&cburnett, 256*3,256, 256,256).to_image();
    let rook_black   = imageops::crop_imm(&cburnett, 256*4,256, 256,256).to_image();
    let pawn_black   = imageops::crop_imm(&cburnett, 256*5,256, 256,256).to_image();

    let piece_lut = [
        [king_white, queen_white, bishop_white, knight_white, rook_white, pawn_white],
        [king_black, queen_black, bishop_black, knight_black, rook_black, pawn_black]
    ];

    let img: RgbaImage = ImageBuffer::new(2048, 2048);

    let mut img = ImageBuffer::from_fn(2048, 2048, |x, y| {
        let cx = x / 256;
        let cy = y / 256;
        if (cx + cy) % 2 == 1 {
            image::Rgba([60u8, 60u8, 60u8, 255u8])
        } else {
            image::Rgba([180u8, 180u8, 180u8, 255u8])
        }
    });

    for x in 0..8 {
        for y in 0..8 {
            let square = unsafe { Square::new(x + y * 8) };
            if let Some(piece) = board.piece_on(square) {
                let p = match piece {
                    Piece::Pawn => 5,
                    Piece::Rook => 4,
                    Piece::Knight => 3,
                    Piece::Bishop => 2,
                    Piece::Queen => 1,
                    Piece::King => 0,
                };
                let c = board.color_on(square).map(|c| if c == Color::White { 0 } else { 1 }).unwrap();
                imageops::overlay(&mut img, &piece_lut[c][p], x as i64 * 256, (7 - y) as i64 * 256);
            }
        }
    }

    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), ImageOutputFormat::Png);
    buf
}
