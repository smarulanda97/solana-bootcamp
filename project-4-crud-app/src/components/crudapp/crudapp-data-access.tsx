'use client'

import { getCrudappProgram, getCrudappProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

interface CreateJournalEntryArgs {
  title: string,
  message: string,
  author: PublicKey
}

type UpdateJournalEntryArgs = CreateJournalEntryArgs;

type DeleteJournalEntry = Omit<CreateJournalEntryArgs, 'message' | 'author' >

export function useCrudappProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getCrudappProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getCrudappProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['crudapp', 'all', { cluster }],
    queryFn: () => program.account.crudapp.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['crudapp', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ crudapp: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  const createJournalEntry = useMutation<string, Error, CreateJournalEntryArgs>({
    mutationKey: ['journal_entry', 'create', { cluster }],
    mutationFn: async ({ title, message, author }) => {
      return program.methods.createJournalEntry(title, message).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      (async () => await accounts.refetch())();
    },
    onError: (error) => {
      toast.error(`Error creating entry: ${error.message}`)
    }
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    createJournalEntry,
    initialize,
  }
}

export function useCrudappProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useCrudappProgram()

  const accountQuery = useQuery({
    queryKey: ['crudapp', 'fetch', { cluster, account }],
    queryFn: () => program.account.crudapp.fetch(account),
  })

  const updateJournalEntry = useMutation<string, Error, UpdateJournalEntryArgs>({
    mutationKey: ['journal_entry', 'update', { cluster }],
    mutationFn: async ({ title, message }) => {
      return program.methods.updateJournalEntry(title, message).rpc();
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      (async () => await accounts.refetch())();
    },
    onError: (error) => {
      toast.error(`Error updating entry: ${error.message}`)
    }
  })

  const deleteJournalEntry = useMutation<string, Error, DeleteJournalEntry>({
    mutationKey: ['journal_entry', 'update', { cluster }],
    mutationFn: async ({ title }) => {
      return program.methods.deleteJournalEntry(title).rpc();
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      (async () => await accounts.refetch())();
    },
    onError: (error) => {
      toast.error(`Error updating entry: ${error.message}`)
    }
  })

  return {
    accountQuery,
    updateJournalEntry,
    deleteJournalEntry,
  }
}
