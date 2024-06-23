import React from 'react';
import "./App.css";

export interface FileItem {
    name: string;
    fullpath: string;
    size: number;
}

export interface DirectoryItem {
    name: string;
    fullpath: string;
    size: number;
    children: FileSystemItem[];
}

export type FileSystemItem =
    | { type: 'FileItem', data: FileItem }
    | { type: 'DirectoryItem', data: DirectoryItem };

interface FileSystemViewProps {
    items: FileSystemItem[];
}

//数値を3桁カンマ区切りの文字列にして返す
const formatNumber = (num: number) => {
    return new Intl.NumberFormat('en-US').format(num);
};

const FileSystemView: React.FC<FileSystemViewProps> = ({ items }) => {
    const renderItem = (item: FileSystemItem) => {
        if (item.type === 'FileItem') {
            return (
                <div className="file">
                    {/* <span className="icon">📄</span> */}
                    <span className="file_name">{item.data.name}</span>
                    {<span>item.data.size</span>  && <span className="file_size">({formatNumber(item.data.size)} bytes)</span>}
                </div>
            );
        } else {
            return (
                <div className="directory">
                    <div className="directory-header">
                        <span className="icon">📂</span>
                        <span className="name">{item.data.name}</span>
                        <span className="dir_size">{formatNumber(item.data.size)} bytes</span>
                    </div>
                    <ul className="directory-contents">
                        {item.data.children.map(renderItem)}
                    </ul>
                </div>
            );
        }
    };
    return <ul>{items.map(renderItem)}</ul>;
};

export default FileSystemView;