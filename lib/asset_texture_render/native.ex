defmodule AssetTextureRender.Native do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :asset_texture_render,
    crate: :texture_render,
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("FORCE_ASSET_TEXTURE_RENDER_BUILD") in ["1", "true"],
    targets: ~w(arm-unknown-linux-gnueabihf aarch64-unknown-linux-gnu aarch64-unknown-linux-musl aarch64-apple-darwin x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-gnu x86_64-pc-windows-msvc),
    version: version

  # When your NIF is loaded, it will override this function.

  @spec role_texture_blend(
          source_path :: String.t(),
          base_img_filename :: String.t(),
          pattern_img_filename :: String.t(),
          pattern_mask_img_filename :: String.t(),
          color_img_filename :: String.t(),
          skin_texture_scale :: {float(), float()},
          change_pattern :: boolean(),
          change_hsv :: boolean(),
          arr_hv_tuple :: {float(), float(), float(), float()}
        ) :: binary()
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

  @spec weapon_texture_blend(
          source_path :: String.t(),
          base_texture_filename :: String.t(),
          pattern_texture_filename :: String.t(),
          mask_texture_filename :: String.t(),
          ao_texture_filename :: String.t(),
          grunge_texture_filename :: String.t(),
          wear_texture_filename :: String.t(),
          default_colors :: {float(), float(), float(), float()},
          wear :: float(),
          seed :: integer(),
          texture_rotate_start :: float(),
          texture_rotate_end :: float(),
          texture_offset_x_start :: float(),
          texture_offset_x_end :: float(),
          texture_offset_y_start :: float(),
          texture_offset_y_end :: float(),
          skin_type :: integer(),
          default_texture :: boolean()
        ) :: binary()
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

end
