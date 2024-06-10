use std::sync::mpsc::Receiver;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::{Context, ContextBuilder, State, TetraError};
use tetra::graphics::mesh::{GeometryBuilder, Mesh};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use crate::dlgo::gotypes::DisplayState;

/// Размер ячейки игрового поля.
const CELL_SIZE: usize = 50;

/// Цвет фона (в случае отсутствия изображения текстуры доски).
const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

/// Цвет сетки.
const MESH_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

/// Цвет текста.
const TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

/// Обозначение ячеек по горизонтали.
const ROW_SYMBOLS: [&'static str; 19]  = ["A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T"];

/// Обозначение ячеек по вертикали.
const COL_NUMBERS: [&'static str; 19] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19"];

/// Служебные размеры доски (для вывода информации).
const SERVICE_WITH: usize = 600;
const SERVICE_HEIGHT: usize = 200;

/// Загрузчик текстур.
struct TextureLoader<'a> {
    ctx: &'a mut Context,
    path: String,
} 

impl<'a> TextureLoader<'a> {
    fn new(ctx: &'a mut Context, base_path: String) -> TextureLoader<'a> {
        TextureLoader {
            ctx,
            path: base_path
        }
    }
    
    fn load(&mut self, filename: &str) -> tetra::Result<Texture> {
        let mut path = self.path.clone();
        path += filename;

        Texture::new(self.ctx, path)   
    }
}

pub struct Window {
    // Канал для получения данных о состоянии мира.
    receiver: Receiver<DisplayState>,

    // Поля, для хранения текстур.
    background_texture: Texture,

    black_stone_texture: Texture,

    greed: Mesh,

    display_state: DisplayState,

    row_text: Vec<Text>,
    col_text: Vec<Text>,
}

impl Window {
    /// Создает новый экземпляр драйвера.
    ///
    /// # Arguments
    ///
    /// * `size`: Размер доски в ячейках.
    /// * `receiver`: Канал для получения данных.
    /// * `asset_path`: Путь к ресурсным файлам.
    /// * `title`: Заглавие окна программы.
    ///
    /// Returns: Result<(), String>
    pub(crate) fn new(
        size: usize,
        receiver: Receiver<DisplayState>,
        resources_path: &str,
        title: &str
    ) -> Result<(), String> {
        let width = size * CELL_SIZE + SERVICE_WITH;
        let height = size * CELL_SIZE + SERVICE_HEIGHT;

        // Пробуем создать контекст.
        let ctx_result = ContextBuilder::new(title, width as i32, height as i32)
            //.high_dpi(true)
            .show_mouse(true)
            .quit_on_escape(true)
            .build();

        let mut ctx = match ctx_result {
            Ok(ctx) => { ctx }
            Err(err) => { return Err(err.to_string()); }
        };

        // Преобразуем в String, поскольку это изменяемый тип.
        let mut resources_path = resources_path.to_owned();
        resources_path.push('/');
       

        // Создаем "состояние".
        let state_result: Result<(), TetraError> = ctx.run(|mut ctx| {
            let background_texture = {
                let mut loader = TextureLoader::new(&mut ctx, resources_path.clone());
                loader.load("background.png")?                
            };


            let mut path = resources_path.clone();
            path.push_str("white_stone_40.png");

            let black_stone_texture = match Texture::new(ctx, path) {
                Ok(t) => { t }
                Err(e) => {
                    return Err(e);
                }
            };

            let mut path = resources_path.clone();
            path.push_str("DejaVuSansMono.ttf");


            let mut col_text: Vec<Text> = vec![];
            let mut row_text: Vec<Text> = vec![];
            
            for i in 0..19 {
                let vector_text = Text::new(
                    COL_NUMBERS[i],
                    Font::vector(ctx, path.clone(), 32.0)?,
                );
                
                col_text.push(vector_text);
            }

            for i in 0..19 {
                let vector_text = Text::new(
                    ROW_SYMBOLS[i],
                    Font::vector(ctx, path.clone(), 32.0)?,
                );

                row_text.push(vector_text);
            }            

            Ok(Window {
                receiver,
                background_texture,
                black_stone_texture,
                display_state: vec![],
                greed: Self::create_greed(ctx, 19_usize).unwrap(),
                row_text,
                col_text,
            })
        });

        match state_result {
            Ok(_) => { Ok(()) }
            Err(err) => { Err(err.to_string()) }
        }
    }

    fn create_greed(ctx: &mut Context, size: usize) -> tetra::Result<Mesh> {
        let stroke_width = 1.0_f32;

        let lines = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, (size * CELL_SIZE) as f32),
            Vec2::new((size * CELL_SIZE) as f32, (size * CELL_SIZE) as f32),
            Vec2::new((size * CELL_SIZE) as f32, 0.0),
            Vec2::new(0.0, 0.0),
        ];

        // If you want to create a `Mesh` with multiple shapes, there is a `GeometryBuilder`
        // type that lets you do this. You can also use it to create buffers, or generate
        // raw vertex data that you can process further yourself.
        let mut complex = GeometryBuilder::new();

        complex.set_color(Color::rgb(0.1, 0.1, 0.1));
        complex.polyline(stroke_width, &lines)?;

        for i in 1..size {
            complex.polyline(stroke_width, &[
                Vec2::new(0.0, (i * CELL_SIZE) as f32),
                Vec2::new((size * CELL_SIZE) as f32, (i * CELL_SIZE) as f32),
            ])?;

            complex.polyline(stroke_width, &[
                Vec2::new((i * CELL_SIZE) as f32, 0.0),
                Vec2::new((i * CELL_SIZE) as f32, (size * CELL_SIZE) as f32),
            ])?;
        }

        let gr = complex.build_mesh(ctx)?;


        Ok(gr)
    }
}

impl State for Window {
    /// Обрабатывает ввод данных от пользователя (клавиатура, мыщ, и т.д.)
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        match self.receiver.try_recv() {
            Ok(state) => {
                self.display_state = state;
            }
            Err(_) => {
                // В канал не передали данные.
            }
        }

        Ok(())
    }

    /// Отображает мир.
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, BACKGROUND_COLOR);

        let mut dp = DrawParams::new();
        dp = dp.position(Vec2::new(0.0, 0.0));
        self.background_texture.draw(ctx, dp);

        self.greed.draw(ctx, Vec2::new(100.0, 100.0));

        let mut dp = DrawParams::new();
        dp = dp.position(Vec2::new(80.0, 80.0));
        self.black_stone_texture.draw(ctx, dp);

        let mut x: usize = 0;
        for text in &mut self.row_text {
            let mut dp = DrawParams::new();
            x += 40;
            
            dp = dp.position(Vec2::new(x as f32, 80.0));
            dp = dp.color(Color::rgb(0.2, 0.2, 0.2));            
            
            text.draw(ctx, dp);
        }

        Ok(())
    }
}


