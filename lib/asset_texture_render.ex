defmodule AssetTextureRender do
  use Rustler,
    otp_app: :asset_texture_render,
    crate: :texture_render

  def role_texture_blend(
    _source_path,
    _base_img_filename,
    _pattern_img_filename,
    _pattern_mask_img_filename,
    _color_img_filename,
    _skin_texture_scale,
    _change_pattern,
    _change_hsv,
    _arr_hv_tuple
  ), do: :erlang.nif_error(:nif_not_loaded)

  def test_role() do
    base_texture_1p = "Char_Veronica_LV3_Cloth"
    skin_texture = "Pattern_test014_C"
    mask_texture_1p = "Char_Veronica_Cloth_Detail_Mask"
    mask_color_1p ="Char_Veronica_Cloth_Color_M3"

    role_texture_blend(

      "Image/",

      base_texture_1p <> "_C.tga",
      skin_texture <> ".tga",
      mask_texture_1p <> ".tga",
      mask_color_1p <> ".tga",

      {0.5, 0.5},
      true,
      true,
      {0.0, 0.1, 0.2, 0.8}
    )
    |> dbg
  end

  def weapon_texture_blend(
    _source_path,
    _base_texture_filename,
    _pattern_texture_filename,
    _mask_texture_filename,
    _ao_texture_filename,
    _grunge_texture_filename,
    _wear_texture_filename,
    _default_colors,
    _wear,
    _seed,
    _texture_rotate_start,
    _texture_rotate_end,
    _texture_offset_x_start,
    _texture_offset_x_end,
    _texture_offset_y_start,
    _texture_offset_y_end,
    _skin_type,
    _default_texture
  ), do: :erlang.nif_error(:nif_not_loaded)

  def test_weapon() do
    resource_name = "Pistol250"
    texture1 = "Pistol250_Camo"

    weapon_texture_blend(
      "WeaponImage/",

      resource_name <> "_C.tga",
      texture1 <> "_C.tga",
      resource_name <> "_Mask.tga",
      resource_name <> "_AO.tga",

      "Gun_Grunge.tga",
      "Paint_Wear.tga",

      {0.0,0.0,0.0,1.0},
      0.4610588,
      666,
      0.0,
      1.0,
      0.0,
      0.6,
      0.0,
      0.8,
      1,
      true
    )
    |> dbg
  end

  # def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
end
