// start command is 'PORT=8080 npm run dev'

import type { Metadata } from "next";
import { Roboto_Mono, Fira_Code, Nunito_Sans, Roboto, Lato, Open_Sans, Inter, Oxygen_Mono, Geist_Mono } from 'next/font/google';
import "./globals.css";

const robotomono = Roboto_Mono({ subsets: ['cyrillic', 'cyrillic-ext', 'greek', 'latin', 'latin-ext', 'vietnamese'], weight: ['100', '200', '300', '400', '500', '600', '700'], variable: '--robotomono-font' });
const firacode = Fira_Code({ subsets: ['latin', 'latin-ext', 'cyrillic', 'cyrillic-ext', 'greek', 'greek-ext'], variable: '--firacode-font' });
const roboto = Roboto({ weight: ['100', '300', '400', '500', '700', '900'], subsets: ['latin', 'latin-ext', 'cyrillic', 'cyrillic-ext', 'greek', 'greek-ext', 'vietnamese'], variable: '--roboto-font' });
const open_sans = Open_Sans({ weight: [ '300', '400', '500', '600', '700', '800' ], subsets: ['latin', 'latin-ext', 'cyrillic', 'cyrillic-ext', 'greek', 'greek-ext', 'hebrew', 'math', 'symbols', 'vietnamese'], variable: '--opensans-font' });
const nunitosans = Open_Sans({ weight: [ '300', '400', '500', '600', '700', '800' ], subsets: ['latin', 'latin-ext', 'cyrillic', 'cyrillic-ext', 'greek', 'greek-ext', 'hebrew', 'math', 'symbols', 'vietnamese'], variable: '--nunitosans-font' });
const oxygen_mono = Oxygen_Mono({ weight: [ '400' ], subsets: ['latin', 'latin-ext'], variable: '--oxygenmono-font' })
const geist_mono = Geist_Mono({ weight: [ '100', '200', '300', '400', '500', '600', '700', '800', '900' ], subsets: ['latin', 'latin-ext'], variable: '--geistmono-font' });

export const metadata: Metadata = {
    title: "hermes",
    description: "messenger for the gods",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className={`${robotomono.variable} ${firacode.variable} ${roboto.variable} ${open_sans.variable} ${nunitosans.variable} ${oxygen_mono.variable} ${geist_mono.variable}`}>
                {children}
            </body>
        </html>
    );
}
