'use client'

import { Keypair, PublicKey } from '@solana/web3.js'
import { useState} from 'react'
import { ExplorerLink } from '../cluster/cluster-ui'
import { useCrudappProgram, useCrudappProgramAccount } from './crudapp-data-access'
import { useWallet } from "@solana/wallet-adapter-react";

export function CrudappCreate() {
  const [title, setTitle] = useState('');
  const [message, setMessage] = useState('');
  const { createJournalEntry } = useCrudappProgram();
  const { publicKey } = useWallet();

  const handleSubmit = () => {
    if (title.trim() === '' || message.trim() === '') {
      alert('error');
      return;
    }

    createJournalEntry.mutateAsync({ title, message, author: publicKey as PublicKey });
  }

  return !publicKey ? (
      <p>Connect your wallet.</p>
    ) : (
    <div>
      <form>
        <div className="py-2">
          <input
            type="text"
            placeholder="Title"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="input input-bordered w-full max-w-xs"
          />
        </div>
        <div className="py-2">
          <textarea
            rows={10}
            placeholder="Message"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            className="input input-bordered w-full max-w-xs"
          />
        </div>
        <button
          type="button"
          onClick={handleSubmit}
          disabled={createJournalEntry.isPending}
          className="btn btn-xs lg:btn-md btn-primary">Create journal entry</button>
      </form>
    </div>
  )
}

export function CrudappList() {
  const {accounts, getProgramAccount} = useCrudappProgram()

  if (getProgramAccount.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }
  if (!getProgramAccount.data?.value) {
    return (
      <div className="alert alert-info flex justify-center">
        <span>Program account not found. Make sure you have deployed the program and are on the correct cluster.</span>
      </div>
    )
  }
  return (
    <div className={'space-y-6'}>
      {accounts.isLoading ? (
        <span className="loading loading-spinner loading-lg"></span>
      ) : accounts.data?.length ? (
        <div className="grid md:grid-cols-2 gap-4">
          {accounts.data?.map((account) => (
            <CrudappCard key={account.publicKey.toString()} account={account.publicKey} />
          ))}
        </div>
      ) : (
        <div className="text-center">
          <h2 className={'text-2xl'}>No accounts</h2>
          No accounts found. Create one above to get started.
        </div>
      )}
    </div>
  )
}

function CrudappCard({ account }: { account: PublicKey }) {
  const { publicKey } = useWallet();
  const { accountQuery, updateJournalEntry, deleteJournalEntry } = useCrudappProgramAccount({
    account,
  });
  const title = accountQuery.data?.title || '';
  const [message, setMessage] = useState('');

  const handleUpdateSubmit = () => {
    if (title.trim() === '' || message.trim() === '') {
      alert('error');
      return;
    }

    updateJournalEntry.mutateAsync({ title, message, author: publicKey as PublicKey });
  }

  const handleDeleteSubmit = () => {
    deleteJournalEntry.mutateAsync({ title });
  }

  return accountQuery.isLoading ? (
    <span className="loading loading-spinner loading-lg"></span>
  ) : (
    <div>
      <form>
        <input
          type="text"
          placeholder="Title"
          value={title}
          readOnly={true}
          className="input input-bordered w-full max-w-xs"
        />
        <textarea
          placeholder="Message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          className="input input-bordered w-full max-w-xs"
        />
        <div className="justify-around">
          <button
            type="button"
            onClick={handleUpdateSubmit}
            disabled={updateJournalEntry.isPending}
            className="btn btn-xs lg:btn-md btn-primary">Update journal entry
          </button>
          <button
            type="button"
            disabled={deleteJournalEntry.isPending}
            className="btn btn-xs lg:btn-md btn-secondary">Delete journal entry
          </button>
        </div>
      </form>
    </div>
  )
}
