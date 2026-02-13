export type LanguageKey =
  | "go"
  | "typescript"
  | "python"
  | "java"
  | "kotlin"
  | "rust"
  | "swift"
  | "dart";

export type LanguageConfig = {
  label: string;
  defaultPackage: string;
  packages: string[];
};

export const LANGUAGE_CONFIG: Record<LanguageKey, LanguageConfig> = {
  go: {
    label: "Go",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "walrus", "seal", "deepbook-v3", "api-reference"],
  },
  typescript: {
    label: "TypeScript",
    defaultPackage: "index",
    packages: ["index"],
  },
  python: {
    label: "Python",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
  java: {
    label: "Java",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
  kotlin: {
    label: "Kotlin",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
  rust: {
    label: "Rust",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
  swift: {
    label: "Swift",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
  dart: {
    label: "Dart",
    defaultPackage: "sui",
    packages: ["sui", "bcs", "deepbook-v3", "walrus", "seal"],
  },
};

export function isLanguageKey(value: string | undefined): value is LanguageKey {
  return Boolean(value && value in LANGUAGE_CONFIG);
}

export function toPackageUrl(language: LanguageKey, pkg: string): string {
  return pkg === "index" ? `/${language}` : `/${language}/${pkg}`;
}
