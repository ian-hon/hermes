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

    const [message, changeMessage] = useState('');
    const [passwordsVerified, changePasswordVerification] = useState(false);

    function attemptLogin(): any {
        sendPostRequest(`${BACKEND_ADDRESS}/user/${userState ? 'login' : 'signup'}`, {
            "username": username,
            "password": password
        }, (r: any) => {
            let response = JSON.parse(r);
            if (response['Success'] != undefined) {
                setCookie("session", response['Success']);
                setCookie("username", username);
                redirect("/");
            } else {
                changeMessage(
                    (): string => {
                        switch (response) {
                            case "UsernameNoExist":
                                return "username doesnt exist";
                            case "PasswordWrong":
                                return "incorrect password";
                            case "UsernameExist":
                                return "username taken";
                            default:
                                return '';
                        }
                    }
                );
            }
        })
    }
    
    function verifyPasswords() {
        if (password.length == 0) {
            changePasswordVerification(false);
        }

        if (password != confirmPassword) {
            changePasswordVerification(false);
        }
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
                        <td>
                            <input type={showPassword ? 'text' : 'password'} value={password} onChange={(e) => { changePassword(e.target.value); verifyPasswords(); }}/>
                            <Image onClick={() => { toggleShowPassword(!showPassword) }} src={showPassword ? '/eye.svg' : '/eye_hide.svg'} alt='' width={25} height={25}></Image>
                        </td>
                    </tr>
                    <tr id={styles.confirmPassword} aria-label={ userState ? 'hide' : 'show' }>
                        <td>
                            <h3>confirm password {'>'} </h3>
                        </td>
                        <td>
                            <input type='password' value={confirmPassword} onChange={(e) => { changeConfirmPassword(e.target.value); verifyPasswords(); }}/>
                        </td>
                    </tr>
                </tbody>
            </table>
            <h3 id={styles.message}>
                { passwordsVerified ? message : (!userState && (password != confirmPassword) ? 'passwords do not match' : message) }
            </h3>
            <div id={styles.action}>
                <h2 onClick={attemptLogin}>
                    [ { userState ? 'log in' : 'sign up' } ]
                </h2>
                <h4 onClick={() => { changeUserState(!userState); changeMessage('') }}>
                    { userState ? 'no account? sign up' : 'have an account? log in' }
                </h4>
            </div>
        </div>
    </div>
}