defmodule AssetTextureRender do
  alias AssetTextureRender.Native

  def role_texture_blend(
    source_path,
    base_img_filename,
    pattern_img_filename,
    pattern_mask_img_filename,
    color_img_filename,
    skin_texture_scale,
    change_pattern,
    change_hsv,
    arr_hv_tuple
  ), do: Native.role_texture_blend(
    source_path,
    base_img_filename,
    pattern_img_filename,
    pattern_mask_img_filename,
    color_img_filename,
    skin_texture_scale,
    change_pattern,
    change_hsv,
    arr_hv_tuple
  )

  def weapon_texture_blend(
    source_path,
    base_texture_filename,
    pattern_texture_filename,
    mask_texture_filename,
    ao_texture_filename,
    grunge_texture_filename,
    wear_texture_filename,
    default_colors,
    wear,
    seed,
    texture_rotate_start,
    texture_rotate_end,
    texture_offset_x_start,
    texture_offset_x_end,
    texture_offset_y_start,
    texture_offset_y_end,
    skin_type,
    default_texture
  ), do: Native.weapon_texture_blend(
    source_path,
    base_texture_filename,
    pattern_texture_filename,
    mask_texture_filename,
    ao_texture_filename,
    grunge_texture_filename,
    wear_texture_filename,
    default_colors,
    wear,
    seed,
    texture_rotate_start,
    texture_rotate_end,
    texture_offset_x_start,
    texture_offset_x_end,
    texture_offset_y_start,
    texture_offset_y_end,
    skin_type,
    default_texture
  )

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

end
