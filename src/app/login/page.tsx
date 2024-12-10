"use client";

import { useState } from 'react';
import styles from './login.module.css';
import Image from 'next/image';
import { BACKEND_ADDRESS } from '../constants';
import { sendPostRequest, setCookie } from '../utils';
import { redirect } from 'next/navigation';

export default function loginPage() {
    const [userState, changeUserState] = useState(true);

    const [username, changeUsername] = useState('lorem');
    const [password, changePassword] = useState('ipsum');
    const [confirmPassword, changeConfirmPassword] = useState('');

    const [showPassword, toggleShowPassword] = useState(false);

    function attemptLogin(): any {
        sendPostRequest(`${BACKEND_ADDRESS}/user/login`, {
            "username": username,
            "password": password
        }, (r: any) => {
            let response = JSON.parse(r);
            if (response['Success'] != undefined) {
                setCookie("session", response['Success']);
                setCookie("username", username);
                // localStorage.setItem("session", response['Success']);
                // localStorage.setItem("username", username);
                redirect("/");
            }
        })
    }

    function placeholder() {
        return <div id={styles.main}>
        <div id={styles.container}>
            <div id={styles.header}>
                <Image src={'/hermes_icon.png'} alt='' width={150} height={150}></Image>
                <h1>
                    hermes
                </h1>
                <h4>
                    messenger for the gods
                </h4>
            </div>
            <div id={styles.userFields}>
                <input id={styles.username} value={username} onChange={(e) => changeUsername(e.target.value)} placeholder="username"/>
                <div>
                    <input type={ showPassword ? "text" : "password" } id={styles.password} value={password} onChange={(e) => changePassword(e.target.value)} placeholder="password"/>
                    <Image onClick={() => { toggleShowPassword(!showPassword) }} src={ showPassword ? "/eye.svg" : "/eye_hide.svg"} alt='' height='50' width='50'/>
                </div>
                <div>
                    <input type="password" aria-label={ userState ? 'log' : 'sign' } id={styles.confirmPassword} value={confirmPassword} onChange={(e) => changeConfirmPassword(e.target.value)} placeholder="confirm your password"/>
                </div>
                <div id={styles.actions}>
                    <h2 onClick={attemptLogin}>
                        { userState ? 'log in' : 'sign up' }
                    </h2>
                    <h4 onClick={() => { changeUserState(!userState); }}>
                        { userState ? 'no account? sign up' : 'have an account? log in' }
                    </h4>
                </div>
            </div>
        </div>
    </div>;
    /*
#main {
    display:flex;
    justify-content: center;
    align-items: center;

    height:100vh;
    width:100vw;
}

#container {
    display:flex;
    justify-content: space-between;
    align-items: center;

    flex-direction: column;

    min-width:40vw;
    min-height:50vh;

    padding: 8vh 10vw 8vh 10vw;

    border-style: solid;
    border-width: var(--border-width);

    & #header {
        display:flex;
        justify-content: center;
        align-items: center;

        flex-direction: column;

        & img {
            height:5em;
            width:5em;

            margin-bottom:1em;
        }

        & h1 {
            font-weight:500;
        }
    
        & h4 {
            font-weight:400;
            font-style:italic;
            opacity: 0.5;
        }
    }

    & #userFields {
        display:flex;
        justify-content: center;
        align-items:center;
        flex-direction: column;

        width:100%;

        margin-top:2.5em;

        & > div {
            margin-top: 1em;

            &:has(#password) {
                display:flex;
                justify-content: center;
                align-items: center;

                position:relative;

                & img {
                    position:absolute;
                    right:1ch;

                    margin-top:1em;

                    height: 1.5em;
                    width: 1.5em;

                    opacity: 0.5;

                    cursor:pointer;
                }
            }
        }

        & input {
            border-collapse: collapse;
            border-style: none;
            background:none;

            color:var(--text);

            font-weight:400;
            outline: none;

            margin-top: 1em;

            height:1.5em;

            border-style: solid;
            border-width:0;
            border-bottom-width:1px;

            transition-duration:0.3s;

            &#confirmPassword {
                height:0;
                margin-top:-1em;
    
                opacity: 0;
    
                transition-duration:0.3s;
            
                &[aria-label='sign'] {
                    opacity: 1;
                    margin-top: 0;
                    height:1.5em;
                }
            }
    
            &:last-child {
                margin-top: 0;
            }
    
            &::placeholder {
                opacity: 0.5;
                font-style:italic;
                font-size:18px;
            }

            &:focus {
                border-color:var(--text);
            }
        }

        & #actions {
            display:flex;
            justify-content: center;
            align-items:center;
            flex-direction: column;

            margin-top:2.5em;

            & h2 {
                background: var(--secondary);

                padding:0.3em 2ch;
                border-radius:var(--border-radius);

                font-weight:400;

                cursor:pointer;
                user-select: none;
                -webkit-user-select: none;

                transition-duration:0.2s;

                &:hover {
                    transform:translateY(-5px);
                }

                &:active {
                    opacity: 0.8;
                    transform:scale(0.98) translateY(-5px);
                }
            }

            & h4 {
                font-weight:600;
                font-style:italic;
                opacity: 0.5;

                margin-top: 0.3em;

                cursor:pointer;

                transition-duration: 0.3s;

                user-select: none;
                -webkit-user-select: none;

                &:hover {
                    opacity: 1;
                }
            }
        }
    }
}
     */
    }
    
    return <div id={styles.main}>
        <div id={styles.container}>
            <div id={styles.header}>
                <Image src={'/hermes_icon.png'} alt='' width={150} height={150}></Image>
                <h1>
                    hermes
                </h1>
                <h4>
                    messenger for the gods
                </h4>
            </div>
            <table id={styles.userFields}>
                <tbody>
                    <tr id={styles.username}>
                        <td>
                            <h3>username {'>'} </h3>
                        </td>
                        <td>
                            <input value={username} onChange={(e) => { changeUsername(e.target.value); }}/>
                        </td>
                    </tr>
                    <tr id={styles.password}>
                        <td>
                            <h3>password {'>'} </h3>
                        </td>
                        {/* poor password protection? */}
                        <td>
                            {/* TODO : password is replaced with * when toggled */}
                            <input value={showPassword ? password : ('*'.repeat(password.length))} onChange={(e) => { changePassword(e.target.value); }}/>
                            <Image onClick={() => { toggleShowPassword(!showPassword) }} src={showPassword ? '/eye.svg' : '/eye_hide.svg'} alt='' width={25} height={25}></Image>
                        </td>
                    </tr>
                    <tr id={styles.confirmPassword} aria-label={ userState ? 'show' : '' }>
                        <td>
                            <h3>confirm password {'>'} </h3>
                        </td>
                        <td>
                            <input value={'*'.repeat(confirmPassword.length)} onChange={(e) => { changeConfirmPassword(e.target.value); }}/>
                        </td>
                    </tr>
                </tbody>
            </table>
            <div id={styles.action}>
                <h2 onClick={attemptLogin}>
                    [ { userState ? 'log in' : 'sign up' } ]
                </h2>
                <h4 onClick={() => { changeUserState(!userState); }}>
                    { userState ? 'no account? sign up' : 'have an account? log in' }
                </h4>
            </div>
        </div>
    </div>
}