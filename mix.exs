defmodule AssetTextureRender.MixProject do
  use Mix.Project
  @version "0.1.2"
  @source_url "https://github.com/KhalilWang/texture_render_nif"

  def project do
    [
      app: :asset_texture_render,
      version: @version,
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.7.1"},
      {:rustler, ">= 0.0.0", optional: true},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end


  defp package do
    [
      maintainers: ["KhalilWang"],
      name: :asset_texture_render,
      licenses: ["MIT"],
      description: "an example for rustler and rustler_precompiled elixir otp app used for render texture",
      files: ["lib",
              "native",
              "checksum-*.exs",
              "mix.exs",
              ".formatter.exs",
              "README*",
              "LICENSE*"
              ],
      exclude_patterns: [
        "target"
      ],
      links: %{"GitHub" => @source_url}
    ]
  end
end
