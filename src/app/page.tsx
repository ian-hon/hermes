"use client";

import Image from "next/image";
import styles from "./page.module.css";
import Link from "next/link";
import React, { useEffect, useState } from "react";
import { redirect } from "next/navigation";
import { fetchCookie } from "./utils";

export default function Home() {
    const [username, changeUsername] = useState<string | null>(null);
    const [sessionID, changeSessionID] = useState<string | null>(null);

    useEffect(() => {
        let u = fetchCookie('username');
        changeUsername(u);
        // because the state doesnt immediately update???
        // and they couldnt just give setState a callback????????
        changeSessionID(fetchCookie('session'));

        if (u == null) {
            redirect("login");
        }
    }, []);

    function placeholder(): React.JSX.Element {
        return <div id={styles.main}>
        <h2>
            hermes is still being constructed...
        </h2>
        <Link id={styles.github_repo} href="https://www.github.com/ian-hon/hermes">
            <Image src={`/sns/github_light.svg`} width={100} height={100} alt=''></Image>
            <h3>ian-hon/hermes</h3>
        </Link>
    </div>
    /*
#main {
    display:flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;

    background:var(--background);
    width:100vw;
    height:100vh;

    & h2 {
        font-family:var(--robotomono-font);
        font-weight:400;
    }

    & #github_repo {
        background:#222;

        display:flex;
        justify-content: center;
        align-items: center;
        flex-direction: row;

        font-size:18px;

        margin-top:2ch;
        padding:0.8ch 0.8em;
        border-radius:1000px;

        border-style:solid;

        border-width:1.5px;
        border-color:#555;
        transition-duration:0.3s;

        cursor:pointer;

        &:hover {
            & h3 {
                text-decoration-color: #fff;
            }
        }

        & h3 {
            font-family:var(--robotomono-font);
            font-weight:400;

            font-size: inherit;

            text-decoration-color:#0000;

            text-decoration: underline;
            text-decoration-thickness: 1px;
            text-underline-offset: 0.1em;

            transition-duration:0.4s;
            transition-timing-function: cubic-bezier(0.075, 0.82, 0.165, 1);
        }

        & img {
            height:1.5em;
            width:1.5em;

            padding:0;
            margin:0;

            margin-right:1ch;
        }
    }
}
     */
    }

    return <div id={styles.main}>
        
    </div>
}
