"use client";

import Image from "next/image";
import styles from "./page.module.css";
import Link from "next/link";
import React, { useEffect, useState } from "react";
import { redirect } from "next/navigation";
import { fetchCookie, sendPostRequest, sessionObject } from "./utils";

import { Channel } from './hermes_types';
import { BACKEND_ADDRESS } from "./constants";

export default function Home() {
    const [username, changeUsername] = useState<string | undefined>(undefined);
    const [sessionID, changeSessionID] = useState<string | undefined>(undefined);

    const [channelMap, updateChannelMap] = useState<Map<number, Channel>>(new Map());

    const [activeChannelID, selectChannelID] = useState<number | undefined>(undefined);
    const [activeChannel, updateChannel] = useState<Channel | undefined>(undefined);

    useEffect(() => {
        let u = fetchCookie('username');
        let s = fetchCookie('session');
        if ((u == undefined) || (s == undefined)) {
            redirect("login");
        }

        changeUsername(u);
        // because the state doesnt immediately update???
        // and they couldnt just give setState a callback????????
        changeSessionID(s);

        const fetchData = async () => {
            await sendPostRequest(`${BACKEND_ADDRESS}/channel/fetch/all`, sessionObject(s), (r: string) => {
                let c: Array<Channel> = JSON.parse(r);

                let m = new Map();
                c.forEach((e) => {
                    m.set(e.id, e);
                });
                updateChannelMap(m);
            });
        }

        fetchData();
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

    function changeChannel(id: number | undefined) {
        // channelList[activeChannelID]
        selectChannelID(id);
        updateChannel(id == undefined ? undefined : channelMap.get(id));
    }

    return <div id={styles.main}>
        <div id={styles.channelList}>
            <div id={styles.container} className={styles.container}>
                <h2>
                    channels
                </h2>
                <div>
                    {
                        Array.from(channelMap.entries()).map((e) =>
                            <div aria-label={ activeChannelID == e[1].id ? 'selected' : '' } className={styles.channel} onClick={() => { changeChannel(activeChannelID == e[1].id ? undefined : e[1].id) }} key={e[1].id}>
                                <div>
                                    <div/>
                                    <div/>
                                </div>
                                <h3>
                                    {e[1].name}
                                </h3>
                            </div>
                        )
                    }
                    {/* {
                        channelMap.entries().map((e) => {
                            return <div aria-label={activeChannelID == e[1].id ? 'selected' : '' } className={styles.channel} onClick={() => { selectChannelID(e[1].id) }} key={e[1].id}>
                                <div>
                                    <div/>
                                    <div/>
                                </div>
                                <h3>
                                    {e[1].name}
                                </h3>
                            </div>;
                        })
                    } */}
                </div>
            </div>
            <div id={styles.userActions} className={styles.container}>
                <h2>
                    {username}
                </h2>
                <div>
                    <Image id={styles.gear} src={'./gear.svg'} alt='' width={25} height={25}></Image>
                    <div onClick={() => { redirect('login') }}>
                        <Image src={'./logout_door.svg'} alt='' width={25} height={25}></Image>
                        <Image src={'./logout_arrow.svg'} alt='' width={25} height={25}></Image>
                    </div>
                </div>
            </div>
        </div>
        <div id={styles.messageBox} className={styles.container}>
            <div id={styles.container}>

            </div>
            <div id={styles.textbox}>

            </div>
        </div>
        <div id={styles.memberList} className={styles.container}>
            <div id={styles.channelInfo}>
                <h2>
                    {
                        activeChannel == undefined ? 'no channel active' : activeChannel.name
                    }
                </h2>
            </div>
        </div>
    </div>
}
