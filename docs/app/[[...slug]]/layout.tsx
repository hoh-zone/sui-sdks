import { DocsLayout } from "fumadocs-ui/layouts/docs";
import type { ReactNode } from "react";

import { baseOptions } from "@/app/layout.config";
import { SidebarFilters } from "@/components/sidebar-filters";
import {
  isLanguageKey,
  LANGUAGE_CONFIG,
  LanguageKey,
  toPackageUrl,
} from "@/lib/docs-filters";
import { source } from "@/lib/source";

function filterTreeByLanguage(pageTree: any, language: LanguageKey) {
  const languageNode = pageTree.children.find(
    (node: any) => node.type === "folder" && node.index?.url === `/${language}`,
  );

  return languageNode;
}

function filterTreeByPackage(
  languageNode: any,
  language: LanguageKey,
  pkg: string,
) {
  if (!languageNode) return undefined;

  const targetUrl = toPackageUrl(language, pkg);
  const packageNode = languageNode.children.find((node: any) => {
    if (node.type === "folder") return node.index?.url === targetUrl;
    if (node.type === "page") return node.url === targetUrl;
    return false;
  });

  if (!packageNode) return undefined;

  return {
    ...languageNode,
    children: [packageNode],
  };
}

export default async function Layout({
  children,
  params,
}: {
  children: ReactNode;
  params: Promise<{ slug?: string[] }>;
}) {
  const { slug } = await params;
  const selectedLanguage: LanguageKey = isLanguageKey(slug?.[0])
    ? slug[0]
    : "go";
  const selectedPackage =
    slug?.[1] && LANGUAGE_CONFIG[selectedLanguage].packages.includes(slug[1])
      ? slug[1]
      : LANGUAGE_CONFIG[selectedLanguage].defaultPackage;

  const languageNode = filterTreeByLanguage(source.pageTree, selectedLanguage);
  const packageTree = filterTreeByPackage(
    languageNode,
    selectedLanguage,
    selectedPackage,
  );
  const filteredTree = packageTree ?? languageNode ?? source.pageTree;

  return (
    <DocsLayout
      {...baseOptions}
      tree={filteredTree}
      sidebar={{
        tabs: (Object.keys(LANGUAGE_CONFIG) as LanguageKey[]).map(
          (language) => ({
            title: `${LANGUAGE_CONFIG[language].label} SDK`,
            url: toPackageUrl(
              language,
              LANGUAGE_CONFIG[language].defaultPackage,
            ),
          }),
        ),
        banner: (
          <SidebarFilters language={selectedLanguage} pkg={selectedPackage} />
        ),
      }}
    >
      {children}
    </DocsLayout>
  );
}
