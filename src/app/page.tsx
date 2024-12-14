"use client";

import Image from "next/image";
import styles from "./page.module.css";
import Link from "next/link";
import React, { useEffect, useState } from "react";
import { redirect } from "next/navigation";
import { fetchCookie, sendPostRequest, sessionObject } from "./utils";

import GearIcon from './assets/gear.svg';
import LogoutArrowIcon from './assets/logout_arrow.svg';
import LogoutDoorIcon from './assets/logout_door.svg';

import { Channel, Message } from './hermes_types';
import { BACKEND_ADDRESS, WS_BACKEND_ADDRESS } from "./constants";
import { checkSession } from "./session_util";

export default function Home() {
    const [username, changeUsername] = useState<string | undefined>(undefined);
    const [sessionID, changeSessionID] = useState<string | undefined>(undefined);

    const [channelMap, updateChannelMap] = useState<Map<number, Channel>>(new Map());

    const [activeChannelID, selectChannelID] = useState<number | undefined>(undefined);
    const [activeChannel, updateChannel] = useState<Channel | undefined>(undefined);

    const [ws, changeWs] = useState<WebSocket | undefined>(undefined);
    const [messages, changeMessages] = useState<Array<Message>>([]);
    const [userInput, changeUserInput] = useState('');

    ws?.addEventListener("open", () => {

    })

    ws?.addEventListener("close", () => {
        changeMessages([]);
    })

    ws?.addEventListener("message", (m) => {
        let r;

        try {
            r = JSON.parse(m.data);
        } catch (error) {
            console.log(`e : ${error}`);
            return;
        }

        if (r.Typical != undefined) {
            changeMessages([...messages, r.Typical]);
        }
    })

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

                checkSession(JSON.parse(r));

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
        changeWs(id == undefined ? undefined : new WebSocket(`${WS_BACKEND_ADDRESS}/message/ws?channel_id=${id}&session_id=${sessionID}`));
        changeMessages([]);

        if ((id != undefined) && (sessionID != undefined)) {
            sendPostRequest(`${BACKEND_ADDRESS}/message/fetch?channel_id=${id}&amount=50`, sessionObject(sessionID), (r: any) => {
                changeMessages(JSON.parse(r).toReversed());
            })
        }
    }

    function Messages({}): React.JSX.Element {
        let addGroup = (u: string) => {
            s.push(
                <div className={styles.messageGroup} key={s.length}>
                    <h3 id={styles.username}>
                        {u}
                    </h3>
                    <div id={styles.container}>
                        {
                            c.map((e) => e)
                        }
                    </div>
                </div>
            );
        }

        let s: Array<React.JSX.Element> = [];
        let c: Array<React.JSX.Element> = [];

        let p = '';
        messages.forEach((m) => {
            if (p != m.author) {
                addGroup(p);

                c = [];
            }

            c.push(
                <div className={styles.message} key={m.id}>
                    <h3>
                        :
                    </h3>
                    <h3 key={m.id}>
                        {m.content}
                    </h3>
                </div>
            );

            p = m.author;
        })
        addGroup(p);

        return <>
            {
                s.map((e) => e)
            }
        </>
    }


    function ChannelList({}): React.JSX.Element {
        return <div id={styles.container} className={styles.container}>
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
            </div>
        </div>
    }

    function ChannelInfo({}): React.JSX.Element {
        return <div id={styles.channelInfo}>
            {
                activeChannel == undefined ?
                <h2>no channel active</h2> :
                <>
                    <h2>
                        {activeChannel.name}
                    </h2>
                    <h3>
                        {activeChannel.description}
                    </h3>
                </>
            }
        </div>
    }

    return <div id={styles.main}>
        <div id={styles.channelList}>
            <ChannelList/>
            <div id={styles.userActions} className={styles.container}>
                <h2>
                    {username}
                </h2>
                <div>
                    <Image id={styles.gear} src={GearIcon} alt='' width={25} height={25}></Image>
                    <div onClick={() => { redirect('login') }}>
                        <Image src={LogoutDoorIcon} alt='' width={25} height={25}></Image>
                        <Image src={LogoutArrowIcon} alt='' width={25} height={25}></Image>
                    </div>
                </div>
            </div>
        </div>
        <div id={styles.messageBox} className={styles.container}>
            <div id={styles.container}>
                <Messages/>
            </div>
            <div id={styles.textbox}>
                <hr/>
                <div>
                    <h3>{'>'}</h3>
                    <input value={userInput} onChange={(e) => { changeUserInput(e.target.value) }} onKeyDownCapture={(e) => { 
                        if (e.key == 'Enter') {
                            ws?.send(JSON.stringify({ "content":userInput }));
                            changeUserInput('');
                        }
                    }}>
                    </input>
                </div>
            </div>
        </div>
        <div id={styles.memberList} className={styles.container}>
            <ChannelInfo/>
        </div>
    </div>
}
