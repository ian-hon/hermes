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
import CrossIcon from './assets/cross.svg';

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
    const [repliedMessage, changeRepliedMessage] = useState<Message | undefined>(undefined);

    const [popup, changePopup] = useState('');

    ws?.addEventListener("open", () => {
        console.log('websocket opened');
    })

    ws?.addEventListener("message", (m) => {
        let r;

        try {
            r = JSON.parse(m.data);
        } catch (error) {
            return;
        }

        if (r.Typical != undefined) {
            changeMessages([...messages, r.Typical]);
            return;
        }
    })

    ws?.addEventListener("error", () => {
        ws.close();
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

        fetchChannels(s);
    }, []);

    const fetchChannels = async (s: string) => {
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
        ws?.close();
        selectChannelID(id);
        updateChannel(id == undefined ? undefined : channelMap.get(id));
        console.log(activeChannel);
        console.log(id, sessionID);
        changeWs(id == undefined ? undefined : new WebSocket(`${WS_BACKEND_ADDRESS}/message/ws?channel_id=${id}&session_id=${sessionID}`));
        changeMessages([]);

        if ((id != undefined) && (sessionID != undefined)) {
            sendPostRequest(`${BACKEND_ADDRESS}/message/fetch?channel_id=${id}&amount=50`, sessionObject(sessionID), (r: any) => {
                console.log(JSON.parse(r));
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
                <div className={styles.message} key={m.id} id={repliedMessage == m ? styles.replied : ''}>
                    {/* <div id={styles.replyContent}>
                        <h3>
                            {'>'}
                        </h3>
                        <h3>
                            han_yuji : "bla bla bla"
                        </h3>
                    </div> */}
                    <div id={styles.content}>
                        <h3>
                            :
                        </h3>
                        <h3 key={m.id}>
                            {m.content}
                        </h3>
                    </div>
                    <div id={styles.replyButton}>
                        <h4 onClick={() => { changeRepliedMessage(m) }}>
                            reply
                        </h4>
                    </div>
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
            <div id={styles.actions}>
                <h2>
                    channels
                </h2>
                <h2 onClick={() => { changePopup('channel') }}>
                    +
                </h2>
            </div>
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
                    <h3>
                        invite code : {activeChannel.invite.toString(16).padEnd(8, '0').slice(0, 4)}-{activeChannel.invite.toString(16).padEnd(8, '0').slice(4, 8)}
                    </h3>
                </>
            }
        </div>
    }

    function ChannelPopup({}): React.JSX.Element {
        const [joinCode, changeJoinCode] = useState('');
        const [channelTitle, changeChannelTitle] = useState('');
        const [channelDescription, changeChannelDescription] = useState('');

        const joinChannel = async () => {
            if (sessionID == undefined) {
                return;
            }

            let invite = Number.parseInt(joinCode.replaceAll("-", ""), 16);
            if (Number.isNaN(invite)) {
                return;
            }

            sendPostRequest(`${BACKEND_ADDRESS}/membership/join?invite=${invite}`, sessionObject(sessionID), (r: any) => {
                console.log(r);
                fetchChannels(sessionID);
                changePopup('');
            })
        }

        const createChannel = async () => {
            if (sessionID == undefined) {
                return;
            }

            if (channelTitle.length <= 0) {
                return;
            }

            sendPostRequest(`${BACKEND_ADDRESS}/channel/create?name=${encodeURIComponent(channelTitle)}&description=${encodeURIComponent(channelDescription)}`, sessionObject(sessionID), () => {
                fetchChannels(sessionID);
                changePopup('');
            })
        }

        return <div id={styles.channelPopup}>
            <Image onClick={() => { changePopup('') }} id={styles.close} src={CrossIcon} alt='' height={20} width={20}/>
            <div id={styles.sections}>
                <div id={styles.inputGroups}>
                    <h2>join : </h2>
                    <input placeholder="code" value={joinCode} onChange={(e) => { changeJoinCode(e.target.value) }}/>
                </div>
                <h2 onClick={() => { joinChannel() }}>
                    [ join ]
                </h2>
            </div>
            <div id={styles.hr}>
                <hr/>
                <h3>or</h3>
                <hr/>
            </div>
            <div id={styles.sections}>
                <div id={styles.inputGroups}>
                    <h2>name : </h2>
                    <input placeholder="channel name" value={channelTitle} onChange={(e) => { changeChannelTitle(e.target.value) }}/>
                </div>
                <div id={styles.inputGroups}>
                    <h2>description : </h2>
                    <input placeholder="channel description" value={channelDescription} onChange={(e) => { changeChannelDescription(e.target.value) }}/>
                </div>
                <h2 onClick={() => { createChannel() }}>
                    [ create ]
                </h2>
            </div>
        </div>;
    }

    return <div id={styles.parent}>
        <div id={styles.popup} aria-label={popup}>
            <ChannelPopup/>
        </div>
        <div id={styles.main}>
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
                    {
                        repliedMessage == undefined ? <></> : 
                        <>
                            <div id={styles.repliedMessage}>
                                <div>
                                    <h3>replying : </h3>
                                    <h3>
                                        { repliedMessage?.content }
                                    </h3>
                                </div>
                                <Image onClick={() => { changeRepliedMessage(undefined) }} src={CrossIcon} alt='' width={25} height={25} />
                            </div>
                            <hr/>
                        </>
                    }
                    <div>
                        <h3>{'>'}</h3>
                        <input value={userInput} onChange={(e) => { changeUserInput(e.target.value) }} onKeyDownCapture={(e) => { 
                            if (e.key == 'Enter') {
                                ws?.send(JSON.stringify({ "content":userInput, "reply": repliedMessage?.id }));
                                changeUserInput('');
                                changeRepliedMessage(undefined);
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
    </div>
}
