//去掉编译时的警告
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

mod utils; //其它.rs文件中实现 

extern crate image;
use image::{DynamicImage, GenericImage, GenericImageView}; //image 图片库，存取图片文件，处理象素
use chrono::prelude::*; //日期时间库
// use std::{env, fmt::format, option, path::PathBuf, vec};  //启动参数
// use ndarray::{linalg::general_mat_mul, prelude::*};//矩阵运算库 dot

use rustler::{Encoder, Env, Error, Term}; //返回到elixir的部分
use rustler::types::binary::{OwnedBinary};

mod vmatrix;
mod UniformRandom;
//use crate::UniformRandom::UniformRandom;

use serde::{Serialize,Deserialize};

use clap::{Parser};

#[derive(Parser,Debug)]
#[clap(author,version,about,long_about=None)]
struct  ArgsWeapon
{
    #[clap(short,long,default_value="ROLE")]
    opttype:String,

    #[clap(short,long,default_value="1024" )]
    size:u32,

    #[clap(short,long)]
    base_texture:Option<String>,

    #[clap(short,long)]
    patter_texture:Option<String>,

    #[clap(short,long)]
    mask_texture:Option<String>,

    #[clap(short,long)]
    ao_texture:Option<String>,

    #[clap(short,long)]
    grunge_texture:Option<String>,

    #[clap(short,long)]
    wear_texture:Option<String>,

    #[clap(short,long)]
    zipd:bool,
}


#[derive(Parser,Debug)]
#[clap(author,version,about,long_about=None)]
struct  ArgsRole
{
    #[clap(short,long)]
    role:bool,
}

#[derive(Serialize, Deserialize, Debug)] 
struct Person {  
    name: String,  
    age: u32,  
    email: Option<String>,  
}

#[rustler::nif]
pub fn role_texture_blend<'a>(
    env: Env<'a>,
    source_path: String,

    base_img_filename: String,
    pattern_img_filename: String,
    pattern_mask_img_filename: String,
    color_img_filename: String,

    skin_texture_scale: (f32,f32),
    change_pattern: bool,
    change_hsv: bool,

    arr_hv_tuple: (f32,f32,f32,f32)
    //save_file_path: String
) ->  Result<Term<'a>, Error>
{
    println!("role_texture_blend() begin");
    
    let mut base_img_path:String=format!("{}{}", source_path, base_img_filename);
    let mut pattern_img_path:String=format!("{}{}", source_path, pattern_img_filename);
    let mut pattern_mask_img_path:String=format!("{}{}", source_path, pattern_mask_img_filename);
    let mut color_img_path:String=format!("{}{}", source_path, color_img_filename);
    // let mut change_pattern=change_pattern;
    // let mut arr_hv:[f32;4]=[0.0,0.0,0.0,0.0];
    let mut arr_hv:[f32;4]=[arr_hv_tuple.0,arr_hv_tuple.1,arr_hv_tuple.2,arr_hv_tuple.3];
    let change_color:bool=change_hsv;

    // let patter_scale=1.0;
    // let pattern_scale_x:f32=patter_scale *image_pattern.dimensions().0 as f32/image_width as f32;
    // let pattern_scale_y:f32=patter_scale *image_pattern.dimensions().1 as f32/image_height as f32;
    let pattern_scale_x:f32 = skin_texture_scale.0;
    let pattern_scale_y:f32 = skin_texture_scale.1;


    //--------------------------------------------------------
    // let args:Vec<String>=env::args().collect();
    
    // let argc_len=args.len();
    // if argc_len>=5
    // {
    //     println!("My Arag={}",args[0]);
    //     base_img_path=args[1].to_string();
    //     pattern_img_path=args[2].to_string();
    //     pattern_mask_img_path=args[3].to_string();
    //     color_img_path=args[3].to_string();
        
    //     if pattern_img_path.len()==0 || pattern_mask_img_path.len()==0
    //     {
    //         change_pattern=false;
    //     }
    // }
    // arr_hv[0]=0.0;
    // arr_hv[1]=0.1;
    // arr_hv[2]=0.2;
    // arr_hv[3]=0.8;

    let mut image_output=image::open(base_img_path).unwrap(); //2048
    let image_pattern: image::DynamicImage=image::open(pattern_img_path).unwrap(); //1024
    let image_pattern_mask=image::open(pattern_mask_img_path).unwrap();//2048
    let image_color_mask=image::open(color_img_path).unwrap(); //2048
    let image_width=image_output.dimensions().0;
    let image_height=image_output.dimensions().1;
    

    println!("pattern_scale_x={},pattern_scale_y={}",pattern_scale_x,pattern_scale_y);
    println!("Begi,  {}",Local::now());
    let mut patter_mask_pixel;
    let mut new_out_pix;
    let mut color_mask_pix;
    let mut color_pix;

    for x in 0..image_width
    {
        for y in 0..image_height
        {
            if change_pattern
            {
                patter_mask_pixel=image_pattern_mask.get_pixel(x, y);
                if patter_mask_pixel[0]==255
                {
                    image_output.put_pixel(x, y, image_pattern.get_pixel((pattern_scale_x*x as f32)  as u32, (y as f32*pattern_scale_y) as u32));
                }
            }
            
            if change_color
            {
                new_out_pix=image_output.get_pixel(x, y);
                color_mask_pix=image_color_mask.get_pixel(x, y);
                for i in 0..arr_hv.len()
                {
                    if arr_hv[i]>0.0 && color_mask_pix[i]>=2
                    {
                        color_pix=utils::set_hue(new_out_pix[0],new_out_pix[1],new_out_pix[2],arr_hv[i]);
                        color_pix  = utils::lerp_rgb_tuple_255((new_out_pix[0],new_out_pix[1],new_out_pix[2]),color_pix,color_mask_pix[i]);
                        new_out_pix[0]=color_pix.0;
                        new_out_pix[1]=color_pix.1;
                        new_out_pix[2]=color_pix.2;
                    }
                }
                image_output.put_pixel(x,y,new_out_pix);
            }
            
        }
        
    }
    println!("over,  {}",Local::now());
    
    //let result= image_output.save(save_file_path);
//
    //if result.is_ok()       
    //{
    //    println!("Save File ok");
    //}

    let bytes = image_output.into_bytes();
    let mut erl_bin: OwnedBinary = OwnedBinary::new(bytes.len()).unwrap();
    erl_bin
        .as_mut_slice()
        .copy_from_slice(bytes.as_slice());
    Ok(erl_bin.release(env).encode(env))
}

fn test()
{
    let mut random:UniformRandom::UniformRandom=UniformRandom::UniformRandom::new();
    random.set_seed(788);

    let offset_x=random.random_float(0.0,1.0);
    let offset_y=random.random_float(0.0,1.0);
    let rot=random.random_float(0.0,360.0);

    println!("offset_x:\n{:?}",offset_x);
    println!("offset_y:\n{:?}",offset_y);
    println!("rot:\n{:?}",rot);
     
    let matrix= vmatrix::get_pattern_srt_matrix(rot,[1.0,1.0],[offset_x,offset_y]);

    let row1=matrix.get_row(0);
    let row2=matrix.get_row(1);

     
    println!("row2:\n{:?}",row2);
    
}

#[rustler::nif]
pub fn weapon_texture_blend<'a>(
    env: Env<'a>,
    source_path: String,
    base_texture_filename: String,
    pattern_texture_filename: String,
    mask_texture_filename: String,
    ao_texture_filename: String,
    grunge_texture_filename: String,
    wear_texture_filename: String,

    default_colors: (f32,f32,f32,f32),
    wear: f32,
    seed: i32, // 用来随机得出纹理参数的种子

    // 纹理各种参数
    texture_rotate_start: f32,
    texture_rotate_end: f32,
    texture_offset_x_start: f32,
    texture_offset_x_end: f32,
    texture_offset_y_start: f32,
    texture_offset_y_end: f32,

    skin_type: u8,
    default_texture: bool
) ->  Result<Term<'a>, Error>
{
    let mut base_texture_path:String=format!("{}{}", source_path, base_texture_filename);
    let mut pattern_texture_path:String=format!("{}{}", source_path, pattern_texture_filename); 
    let mut mask_texture_path:String=format!("{}{}", source_path, mask_texture_filename);
    let mut ao_texture_path:String=format!("{}{}", source_path, ao_texture_filename);
    let mut grunge_texture_path:String=format!("{}{}", source_path, grunge_texture_filename);
    let mut wear_texture_path:String=format!("{}{}", source_path, wear_texture_filename);
    
    let mut _grunge_color:[f32;4]=[default_colors.0,default_colors.1,default_colors.2,default_colors.3];

    let mut wear_amt:f32=wear;  //wear degree 0.4610588

    let mut custom_color0:[f32;4]=[1.0,1.0,1.0,1.0];  //原配置值，现全为1
    let mut custom_color1:[f32;4]=[1.0,1.0,1.0,1.0];
    let mut custom_color2:[f32;4]=[1.0,1.0,1.0,1.0];
    let mut custom_color3:[f32;4]=[1.0,1.0,1.0,1.0];

    
    let mut random:UniformRandom::UniformRandom=UniformRandom::UniformRandom::new();
    random.set_seed(seed);
    let offset_x=random.random_float(texture_offset_x_start, texture_offset_x_end);
    let offset_y=random.random_float(texture_offset_y_start, texture_offset_y_end);
    let rot=random.random_float(texture_rotate_start, texture_rotate_end);
    let matrix_pattern= vmatrix::get_pattern_srt_matrix(rot, [1.0,1.0], [offset_x,offset_y]);
    let mut pattern_srt0:[f32;4]=matrix_pattern.get_row(0);
    let mut pattern_srt1:[f32;4]=matrix_pattern.get_row(1);
    // let mut pattern_srt0:[f32;4]=[1.0,0.0,0.0,0.0];
    // let mut pattern_srt1:[f32;4]=[0.0,1.0,0.0,0.0];

    let wear_rot = random.random_float(0.0, 360.0);
    let wear_scale = random.random_float(1.6, 1.8);
    let wear_offset_x = random.random_float(0.0, 1.0);
    let wear_offset_y = random.random_float(0.0, 1.0);
    let matrix_wear= vmatrix::get_wear_srt_matrix(wear_rot, wear_scale, [wear_offset_x, wear_offset_y]);
    let mut wear_srt0:[f32;4]=matrix_wear.get_row(0);
    let mut wear_srt1:[f32;4]=matrix_wear.get_row(0);
    // let mut wear_srt0:[f32;4]=[-1.7,0.2,0.0,1.4];
    // let mut wear_srt1:[f32;4]=[-0.2,-1.7,0.0,0.2];
    
    let grunge_rot = random.random_float(0.0, 360.0);
    let grunge_scale = random.random_float(1.6, 1.8);
    let grunge_offset_x = random.random_float(0.0, 1.0);
    let grunge_offset_y = random.random_float(0.0, 1.0);
    let matrix_grunge= vmatrix::get_grunge_srt_matrix(grunge_rot, grunge_scale, [grunge_offset_x, grunge_offset_y]);
    let mut grunge_srt0:[f32;4]=matrix_grunge.get_row(0);
    let mut grunge_srt1:[f32;4]=matrix_grunge.get_row(0);
    // let mut grunge_srt0:[f32;4]=[-1.7, -0.5, 0.0, 2.1]; //grunge:垃圾摇滚风格 Scale Rotation Translation
    // let mut grunge_srt1:[f32;4]=[0.5, -1.7, 0.0, 0.0];

    let mut paint_style:u8=skin_type;
    let mut need_base_texture:bool=default_texture;
    let pattern_texture_st:[f32;4]=[1.0,1.0,0.0,0.0];

    let wear_texture_x = random.random_float(1.6, 1.8);
    let wear_texture_y = random.random_float(1.6, 1.8);
    let wear_texture_st:[f32;4]=[wear_texture_x,wear_texture_y,0.0,0.0];

    let grunge_texture_x = random.random_float(1.6, 1.8);
    let grunge_texture_y = random.random_float(1.6, 1.8);
    let grunge_texture_st:[f32;4]=[grunge_texture_x,grunge_texture_y,0.0,0.0];
    
    /* 
    //-----------------------从参数中取值修改上面变量---------------------------
    let args:Vec<String>=env::args().collect();
    let argc_len=args.len();
    if argc_len>=5
    {
        //根据参数传入重设置上面的参数值
        base_texture_path=args[1].to_string();
    }
    */
    //*************实现******************************** */
    //PAINTSTYL_X  7--手绘， 8--光泽 9--枪匠
    const PAINTSTYL_7:u8=1;
    const PAINTSTYL_8:u8=2;
    const PAINTSTYL_9:u8=3;
    const TEXTURE_SIZE:u32=2048; //处理输出的纹理大小
    const R:usize=0;const G:usize=1;const B:usize=2;const A:usize=3;  //通道索引
    //加载纹理
    let mut image_base:DynamicImage=DynamicImage::new(TEXTURE_SIZE, TEXTURE_SIZE, image::ColorType::Rgb8);//=image::open(base_texture_path).unwrap();  
    let mut image_pattern=image::open(pattern_texture_path).unwrap();  
    let mut image_mask=image::open(mask_texture_path).unwrap();  
    let mut image_ao=image::open(ao_texture_path).unwrap();  
    let mut image_grunge=image::open(grunge_texture_path).unwrap();  
    let mut image_wear=image::open(wear_texture_path).unwrap();  
    let mut image_output:DynamicImage=DynamicImage::new(2048, 2048, image::ColorType::Rgb8);//=image::open(base_texture_path).unwrap();  
 
    if need_base_texture
    {
        image_base=image::open(base_texture_path).unwrap();
    }

    let mut base_color_pixel:image::Rgba<u8>;//=image::Rgba([0,0,0,0]);
    let mut pattern_color_pixel:image::Rgba<u8>;//==image::Rgba([0,0,0,0]);
    let mut mask_color_pixel:image::Rgba<u8>;//==image::Rgba([0,0,0,0]);
    let mut ao_color_pixel:image::Rgba<u8>;//==image::Rgba([0,0,0,0]);
    let mut grunge_color_pixel:image::Rgba<u8>;//==image::Rgba([0,0,0,0]);
    let mut wear_color_pixel:image::Rgba<u8>;//==image::Rgba([0,0,0,0]);
    let mut output_color_pixel:image::Rgba<u8>=image::Rgba([0,0,0,0]);

    let mut target_pixel:image::Rgba<u8>;

    let mut base_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut pattern_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut ao_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut wear_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut mask_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut out_color:[f32;4];//=[0.0,0.0,0.0,0.0];
    let mut grunge_color:[f32;4];
    let mut target_color:[f32;4];
  
    println!("Beg ,{:?}",Local::now());
    for x in 0..TEXTURE_SIZE
    {
        for y in 0..TEXTURE_SIZE
        {
 
            let pattern_xy=get_uv_with_srt([x,y], pattern_srt0, pattern_srt1, pattern_texture_st,TEXTURE_SIZE);
            pattern_color_pixel=image_pattern.get_pixel(pattern_xy[0],pattern_xy[1]);
            
            if need_base_texture  //NEED_BASETEXTURE
            {       
                base_color_pixel=image_base.get_pixel(x, y);
            }
           else 
           {
                base_color_pixel =get_rgba_color_255(_grunge_color);
                base_color_pixel[A]=pattern_color_pixel[A];//.a
            }

            ao_color_pixel=image_ao.get_pixel(x, y);

            let gunge_uv:[u32;2]=get_uv_with_srt([x,y], grunge_srt0, grunge_srt1, grunge_texture_st,TEXTURE_SIZE);
            let wear_uv:[u32;2] =get_uv_with_srt([x,y], wear_srt0,   wear_srt1,   wear_texture_st,  TEXTURE_SIZE);
           
            grunge_color_pixel=image_grunge.get_pixel(gunge_uv[0],gunge_uv[1]);
            wear_color_pixel    =image_wear.get_pixel(wear_uv[0],wear_uv[1]);
            mask_color_pixel    =image_mask.get_pixel(x, y);

            pattern_color=get_rgba_color_01(pattern_color_pixel);
            base_color=get_rgba_color_01(base_color_pixel);
            ao_color=get_rgba_color_01(ao_color_pixel);
            grunge_color=get_rgba_color_01(grunge_color_pixel);
            wear_color=get_rgba_color_01(wear_color_pixel);
            mask_color=get_rgba_color_01(mask_color_pixel);
            

            let mut ao_cavitiy=ao_color[R];      //r
            let mut paint_lerp_factor=ao_color[A]; //a
            let mut paint_wear=wear_color[G];  //g
            if paint_style != PAINTSTYL_8
            {
                paint_lerp_factor=paint_lerp_factor+paint_wear*ao_cavitiy;
                paint_lerp_factor=paint_lerp_factor *(wear_amt*6.0+1.0);
                if paint_style== PAINTSTYL_7 || paint_style== PAINTSTYL_9
                {
                    //let fl_cuttable_area:f32 = 1.0f;
                    paint_lerp_factor = paint_lerp_factor+utils::smoothstep(0.51, 0.61, pattern_color[3])*utils::smoothstep(1.0, 0.89, pattern_color[3]);
                    if paint_style==PAINTSTYL_9
                    {
                        paint_lerp_factor=paint_lerp_factor*utils::cmp_max(0.0, utils::smoothstep(0.0, 0.51, pattern_color[A]));
                        pattern_color[3]=utils::lerp(pattern_color[A], utils::saturate(pattern_color[A]*1.98), mask_color[A]);
                    }
                    else 
                    {
                        paint_lerp_factor=paint_lerp_factor*utils::cmp_max(0.0, utils::smoothstep(0.0, 0.49, 0.5));

                    }
                }
            }


            if paint_style!=PAINTSTYL_8 && paint_style!=PAINTSTYL_9
            {
                paint_lerp_factor=utils::smoothstep(0.58,0.68,paint_lerp_factor);
            }
            else if paint_style==PAINTSTYL_9 {
                paint_lerp_factor=utils::lerp(utils::smoothstep(0.58,0.68,paint_lerp_factor), paint_lerp_factor, mask_color[R]);
            }

            let mut c_base=base_color;
            let mut c_paint: [f32; 4]=custom_color0;
            let mut c_grunge=grunge_color;
            let mut fl_grunge:f32=0.0;
            if paint_style==PAINTSTYL_8 || paint_style==PAINTSTYL_9
            {
                fl_grunge=c_grunge[R]*c_grunge[G]*c_grunge[B];
            }
            c_grunge=utils::lerp_arr4([1.0,1.0,1.0,1.0],c_grunge,(1.0-ao_cavitiy).powf(4.0)*0.26+0.74*wear_amt);

            if paint_style==PAINTSTYL_7
            {
                c_paint[R]=pattern_color[R];c_paint[G]=pattern_color[G];c_paint[B]=pattern_color[B];
            }

            if paint_style==PAINTSTYL_8 || paint_style==PAINTSTYL_9
            {
                let mut patina_lerp_factor:f32=paint_wear*ao_cavitiy*ao_cavitiy;//光泽
                patina_lerp_factor=utils::smoothstep(0.1,0.21,patina_lerp_factor*wear_amt);

                let mut oil_lerp_factor=utils::saturate(ao_cavitiy-wear_amt*0.1)-fl_grunge;
                oil_lerp_factor=utils::smoothstep(0.0,0.15,oil_lerp_factor+0.082);

                let mut c_patina=utils::lerp_arr4(custom_color1,custom_color2,wear_amt);
                let mut c_oilrub_color=utils::lerp_arr4(custom_color1,custom_color3,wear_amt.powf(0.5));
                c_patina= utils::mul_arr4(utils::lerp_arr4(c_oilrub_color,c_patina, oil_lerp_factor),pattern_color);

                let bright_param:[f32;3]=[0.3,0.59,0.11];
                let brightness=vmatrix::arr3_dot([pattern_color[R],pattern_color[G],pattern_color[B]],bright_param);
                
                let scratches:[f32;4]=[custom_color0[R]*brightness,custom_color0[G]*brightness,custom_color0[B]*brightness,0.0];
                c_patina=utils::lerp_arr4(c_patina,scratches, patina_lerp_factor);
                if paint_style==PAINTSTYL_8
                {
                    c_paint=c_patina;
                    paint_lerp_factor=1.0-mask_color[R];
                }
                else if paint_style==PAINTSTYL_9 {
                    c_paint=utils::lerp_arr4(pattern_color,c_patina, mask_color[R]);
                    paint_lerp_factor=paint_lerp_factor*(1.0-mask_color[R]);
                }
            }

            c_paint=utils::mul_arr4(c_paint,c_grunge);
            out_color=utils::lerp_arr4(c_paint, c_base,paint_lerp_factor);
            out_color[A]=c_base[A];

            output_color_pixel[R]=(out_color[R]*255.0) as u8;
            output_color_pixel[G]=(out_color[G]*255.0) as u8;
            output_color_pixel[B]=(out_color[B]*255.0) as u8;
            output_color_pixel[A]=(out_color[A]*255.0) as u8;
            image_output.put_pixel(x,y,output_color_pixel);
            
        }
    }
    
    println!("End ,{:?}",Local::now());
    // let mut file_name:String= format!("WeaponImage/wp_output_{:?}.png",wear_amt);
    // let mut out_string:String="filename=".to_string()+ &file_name ;//+format("{:?}",wear_amt);
    //let result=  image_output.save(save_file_path);
    //if result.is_ok()       
    //{
    //    println!("Save File ok");
    //}
    //else {
    //    println!("Save File fail");
    //}

    println!("Weapon custom texture blend finished!");

    let bytes = image_output.into_bytes();
    let mut erl_bin: OwnedBinary = OwnedBinary::new(bytes.len()).unwrap();
    erl_bin
        .as_mut_slice()
        .copy_from_slice(bytes.as_slice());
    Ok(erl_bin.release(env).encode(env))
}


fn get_uv_with_srt(uv_u32:[u32;2],srt0:[f32;4],srt1:[f32;4],texture_st:[f32;4],texture_size:u32)->[u32;2]
{
    let mut uv2:[f32;2]=[0.0,0.0];
    let uv:[f32;2]=[uv_u32[0] as f32,uv_u32[1] as f32];
    let srt0_xy:[f32;2]=[srt0[0],srt0[1]];
    let srt1_xy:[f32;2]=[srt1[0],srt1[1]];
    uv2[0]=vmatrix::arr2_dot(uv,srt0_xy) + srt0[3]; //uv2.x = dot(uv,srt0.xy)+srt0.w
    uv2[1]=vmatrix::arr2_dot(uv,srt1_xy) + srt1[3]; //uv2.y = dot(uv,srt0.xy)+srtuv
    uv2[1]=uv2[1]+(1.0-texture_st[1]);  //uv2.y += (1 - Texture_ST.y);
    return [uv2[0] as u32 % texture_size,uv2[1] as u32 % texture_size];
}

 //0~1格式转成255格式
fn get_rgba_color_255(color_01:[f32;4])->image::Rgba<u8>
{
    let arr_size=color_01.len();
    let mut out_color:image::Rgba<u8>=image::Rgba([0,0,0,0]);
    for i in 0..arr_size as usize
    {
        out_color[i]=(color_01[i]*255.0) as u8;
        if i>=4
        {
            break;
        }
    }
    return out_color;
}

//255格式转0~1格式
fn get_rgba_color_01(color_255:image::Rgba<u8>)->[f32;4]
{
    let mut out_color:[f32;4]=[0.0,0.0,0.0,0.0];
    for i in 0..4 as usize
    {
        out_color[i]=(color_255[i] as f32/255.0) as f32;
    }
    return out_color;
}

rustler::init!("Elixir.AssetTextureRender", [role_texture_blend, weapon_texture_blend]);