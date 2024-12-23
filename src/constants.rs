pub const AGENTS: [&str; 7] = ["npm", "yarn", "yarn@berry", "pnpm", "pnpm@6", "bun", "deno"];

pub static LOCKS: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "bun.lockb" => "bun",
  "deno.lock" => "deno",
  "pnpm-lock.yaml" => "pnpm",
  "yarn.lock" => "yarn",
  "package-lock.json" => "npm",
  "npm-shrinkwrap.json" => "npm",
};

pub static INSTALL_PAGE: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "bun" => "https://bun.sh",
  "deno" => "https://deno.com",
  "pnpm" => "https://pnpm.io/installation",
  "pnpm@6" => "https://pnpm.io/6.x/installation",
  "yarn" => "https://classic.yarnpkg.com/en/docs/install",
  "yarn@berry" => "https://yarnpkg.com/getting-started/install",
  "npm" => "https://docs.npmjs.com/cli/v8/configuring-npm/install",
};
