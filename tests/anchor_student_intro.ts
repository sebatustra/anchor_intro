import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntro } from "../target/types/anchor_student_intro";
import { assert, expect } from "chai"
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("anchor_student_intro", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.AnchorStudentIntro as Program<AnchorStudentIntro>;

    const commenter = anchor.web3.Keypair.generate();
    
    const intro = {
        name: "Sebastián Jara",
        message: "mensaje de prueba"
    };

    const [introPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(intro.name),
            provider.wallet.publicKey.toBuffer()
        ],
        program.programId
    );

    const [mintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("mint")
        ],
        program.programId
    );

    const introComment = {
        comment: "Buena intro bro"
    };

    const [commentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            commenter.publicKey.toBuffer(),
            introPDA.toBuffer()
        ],
        program.programId
    );

    const [commentCounterPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("counter"),
            introPDA.toBuffer()
        ],
        program.programId
    )

    it("Initialized the reward mint", async () => {
        await program.methods
            .initializeMint()
            .rpc()
    })

    it("Student intro is added", async () => {

        const tokenAccount = await getAssociatedTokenAddress(
            mintPDA,
            provider.wallet.publicKey,
        );

        await program.methods
            .addStudentIntro(
                intro.name,
                intro.message
            )
            .accounts({
                tokenAccount,
            })
            .rpc();

        const account = await program.account.introState.fetch(introPDA);
        expect(account.name).to.equal(intro.name);
        expect(account.message).to.equal(intro.message)

        const userATA = await getAccount(
            provider.connection,
            tokenAccount
        );

        expect(Number(userATA.amount)).to.equal(10)

        const counter = await program.account.commentCounterState.fetch(commentCounterPDA);

        expect(Number(counter.count)).to.equal(0)
        expect(counter.introAccount.toString()).to.equal(introPDA.toString())

    });

    it("Intro is commented", async () => {

        const commenterTokenAccount = await getAssociatedTokenAddress(
            mintPDA,
            commenter.publicKey
        );

        const tx = await provider.connection.requestAirdrop(commenter.publicKey, anchor.web3.LAMPORTS_PER_SOL * 10)
        await provider.connection.confirmTransaction(tx)

        await program.methods
            .addCommentToIntro(introComment.comment)
            .accounts({
                commenter: commenter.publicKey,
                intro: introPDA,
                tokenAccount: commenterTokenAccount
            })
            .signers([commenter])
            .rpc()

        const commentAccount = await program.account.introCommentState.fetch(commentPDA);
        expect(commentAccount.comment).to.equal(introComment.comment);
        expect(commentAccount.commenter.toString()).to.equal(commenter.publicKey.toString())

        const counter = await program.account.commentCounterState.fetch(commentCounterPDA);

        expect(Number(counter.count)).to.equal(1)
        expect(counter.introAccount.toString()).to.equal(introPDA.toString())

        const commenterATA = await getAccount(
            provider.connection,
            commenterTokenAccount
        );

        expect(Number(commenterATA.amount)).to.equal(2)
    })

    it("Student intro is updated", async () => {
        const newMessage = "otro mensaje de prueba";

        await program.methods
            .updateStudentIntro(intro.name, newMessage)
            .rpc()

        const account = await program.account.introState.fetch(introPDA);
        expect(account.name).to.equal(intro.name);
        expect(account.message).to.equal(newMessage)
    })

    it("Student intro is closed", async () => {
        await program.methods
            .closeStudentIntro(intro.name)
            .rpc()
    })
});
