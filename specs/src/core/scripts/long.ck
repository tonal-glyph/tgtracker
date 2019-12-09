<<< "Shred was added" >>>;
// define function
fun void foo()
{
    while( true )
    {
        <<< "hi" >>>;
        .2::second => now;
    }
}
// spork the function, remember the shred reference
spork ~ foo() @=> Shred @ s;
// wait for 2 seconds
2::second => now;
<<<"Removing shred by id">>>;
// remove by id
Machine.remove( s.id() );

// wait a little more
2::second => now;

10.0 :: second => dur myDur; // long running shred
myDur => now;