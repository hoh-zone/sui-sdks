import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import type { ReactNode } from 'react';

import { baseOptions } from '@/app/layout.config';
import { source } from '@/lib/source';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout
      {...baseOptions}
      tree={source.pageTree}
      sidebar={{
        tabs: [
          {
            title: 'Sui SDK',
            description: 'Go interfaces for Sui',
            url: '/sui'
          },
          {
            title: 'BCS',
            description: 'Binary Canonical Serialization in Go',
            url: '/bcs'
          },
          {
            title: 'Walrus',
            description: 'Walrus SDK in Go',
            url: '/walrus'
          },
          {
            title: 'Seal',
            description: 'Seal cryptography SDK in Go',
            url: '/seal'
          },
          {
            title: 'DeepBook V3',
            description: 'DeepBook V3 bindings in Go',
            url: '/deepbook-v3'
          },
          {
            title: 'API Reference',
            url: '/api-reference'
          }
        ]
      }}
    >
      {children}
    </DocsLayout>
  );
}
