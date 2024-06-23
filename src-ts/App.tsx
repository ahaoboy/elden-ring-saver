// biome-ignore lint/style/useImportType: <explanation>
import React, { useMemo, useState } from 'react';
import './index.css';
import { DownloadOutlined, InboxOutlined } from '@ant-design/icons';
import type { UploadProps } from 'antd';
import { Button, message, Select, Upload } from 'antd';
import "./App.css"
import { type Slot, get_slots, replace_slot, } from './wasm';

const { Dragger } = Upload;
async function download(name: string, buf: Uint8Array) {
  const blob = new Blob([buf], { type: 'application/octet-stream' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.click();
}

async function readFile(file: File): Promise<Uint8Array> {
  return new Promise(r => {
    const reader = new FileReader()

    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    reader.onload = (e: any) => {
      const arrayBuffer = e.target.result;
      const uint8Array = new Uint8Array(arrayBuffer);
      r(uint8Array)
    }
    reader.readAsArrayBuffer(file)

  })
}


function get_slot_item(i: Slot, k: number) {
  return {
    value: k,
    active: i.active,
    label: `${i.character_name} (lv${i.character_level})`
  }
}

const App: React.FC = () => {
  const [source, setSource] = useState<Uint8Array | undefined>(undefined)
  const [targetName, setTargetName] = useState('')
  const [target, setTarget] = useState<Uint8Array | undefined>(undefined)
  const [sourceIndex, setSourceIndex] = useState<undefined | number>(undefined)
  const [targetIndex, setTargetIndex] = useState<undefined | number>(undefined)
  const [sourceSlot, setSourceSlot] = useState<Slot[]>([])
  const [targetSlot, setTargetSlot] = useState<Slot[]>([])

  const sourceOption = sourceSlot.map(get_slot_item)
  const targetOption = targetSlot.map(get_slot_item)

  const canDownload = sourceIndex !== undefined && sourceIndex >= 0 && targetIndex !== undefined && targetIndex >= 0

  return <div className='main'>
    <div className='item'>
      <h2>Source</h2>
      <Dragger
        accept=".sl2"
        name='source file'
        maxCount={1}
        multiple={false}
        customRequest={async (e) => {
          const file = e.file as File
          const buf = await readFile(file)
          setSource(buf)
          const slot = get_slots(buf)
          setSourceSlot(slot)
          setSourceIndex(slot.findIndex(i => i.active))
          e.onSuccess?.(undefined)
        }}
      >
        <p className="ant-upload-drag-icon">
          <InboxOutlined />
        </p>
        <p className="ant-upload-text">Click or drag source file to this area to upload</p>
      </Dragger>
      <Select
        value={sourceIndex}
        placeholder="select source slot"
        style={{ width: "100%" }}
        onSelect={(e) => {
          setSourceIndex(e)
        }}
        options={sourceOption}
      />
    </div>
    <div className='item'>
      <p>Copy Slot from Source to target</p>
      <Button disabled={!canDownload} type="primary" icon={<DownloadOutlined />} size={"large"}

        onClick={() => {
          if (!canDownload) {
            return
          }
          // biome-ignore lint/style/noNonNullAssertion: <explanation>
          const buf = replace_slot(target!, targetIndex, source!, sourceIndex)
          download(targetName, buf)
        }}

      >
        Download
      </Button>
    </div>
    <div className='item'>
      <h2>Target</h2>
      <Dragger
        accept=".sl2"
        name='target file'
        maxCount={1}
        multiple={false}
        customRequest={async (e) => {
          const file = e.file as File;
          setTargetName(file.name)
          const buf = await readFile(file as File)
          setTarget(buf)
          const slot = get_slots(buf)
          setTargetIndex(slot.findIndex(i => i.active))
          setTargetSlot(slot)
          e.onSuccess?.(undefined)
        }}
      >
        <p className="ant-upload-drag-icon">
          <InboxOutlined />
        </p>
        <p className="ant-upload-text">Click or drag target file to this area to upload</p>
      </Dragger>
      <Select
        value={targetIndex}
        placeholder="select target slot"
        style={{ width: "100%" }}
        onSelect={(e) => {
          setTargetIndex(e)
        }}
        options={targetOption}
      />
    </div>
  </div >

}

export default App;