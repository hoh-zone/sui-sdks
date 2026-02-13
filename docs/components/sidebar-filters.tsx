"use client";

import { useRouter } from "next/navigation";
import { ChangeEvent } from "react";

import {
  LANGUAGE_CONFIG,
  LanguageKey,
  toPackageUrl,
} from "@/lib/docs-filters";

type SidebarFiltersProps = {
  language: LanguageKey;
  pkg: string;
};

export function SidebarFilters({ language, pkg }: SidebarFiltersProps) {
  const router = useRouter();
  const languageOptions = Object.keys(LANGUAGE_CONFIG) as LanguageKey[];
  const packageOptions = LANGUAGE_CONFIG[language].packages;

  const onLanguageChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const nextLanguage = event.target.value as LanguageKey;
    const nextPackage = LANGUAGE_CONFIG[nextLanguage].defaultPackage;
    router.push(toPackageUrl(nextLanguage, nextPackage));
  };

  const onPackageChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const nextPackage = event.target.value;
    router.push(toPackageUrl(language, nextPackage));
  };

  return (
    <div className="space-y-2 rounded-lg border border-fd-border bg-fd-card p-2">
      <div className="space-y-1.5">
        <label
          className="block text-xs font-medium text-fd-muted-foreground"
          htmlFor="language-filter"
        >
          编程语言
        </label>
        <select
          id="language-filter"
          className="w-full rounded-md border border-fd-border bg-fd-background px-2 py-1.5 text-sm"
          value={language}
          onChange={onLanguageChange}
        >
          {languageOptions.map((option) => (
            <option key={option} value={option}>
              {LANGUAGE_CONFIG[option].label}
            </option>
          ))}
        </select>
      </div>

      <div className="space-y-1.5">
        <label
          className="block text-xs font-medium text-fd-muted-foreground"
          htmlFor="package-filter"
        >
          包
        </label>
        <select
          id="package-filter"
          className="w-full rounded-md border border-fd-border bg-fd-background px-2 py-1.5 text-sm"
          value={pkg}
          onChange={onPackageChange}
        >
          {packageOptions.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
