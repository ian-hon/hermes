"use client";

import Image from "next/image";
import styles from "./page.module.css";
import Link from "next/link";
import React, { useState } from "react";
import { redirect } from "next/navigation";

export default function Home() {
    const [username, changeUsername] = useState(localStorage.getItem("username"));
    const [sessionID, changeSessionID] = useState(localStorage.getItem("session"));

    // if (username == undefined) {
    //     redirect("login");
    // }

    return <div id={styles.main}>
        <h2>
            hermes is still being constructed...
        </h2>
        <Link id={styles.github_repo} href="https://www.github.com/ian-hon/hermes">
            <Image src={`/sns/github_light.svg`} width={100} height={100} alt=''></Image>
            <h3>ian-hon/hermes</h3>
        </Link>
    </div>
}
